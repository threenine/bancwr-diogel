use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use crate::signer::Signer;
use crate::db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use tracing::{info, warn};

/// NIP-46 JSON-RPC request
#[derive(Debug, Deserialize, Serialize)]
pub struct Nip46Request {
    pub id: String,
    pub method: String,
    pub params: Vec<String>,
}

/// NIP-46 JSON-RPC response
#[derive(Debug, Serialize, Deserialize)]
pub struct Nip46Response {
    pub id: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClientPermissions {
    pub allowed_methods: Vec<String>,  // e.g., ["sign_event:1", "nip44_encrypt"]
    pub connected_at: DateTime<Utc>,
}

/// NIP-46 protocol handler
pub struct Nip46Handler {
    signer: Arc<RwLock<Signer>>,
    db: Database,
    // Track connected clients: client_pubkey -> permissions
    connections: Arc<RwLock<HashMap<PublicKey, ClientPermissions>>>,
}

impl Nip46Handler {
    pub fn new(signer: Arc<RwLock<Signer>>, db: Database) -> Self {
        Self {
            signer,
            db,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
    
    /// Handle incoming NIP-46 request
    pub async fn handle_request(
        &self,
        request: Nip46Request,
        client_pubkey: PublicKey,
    ) -> Nip46Response {
        info!("Handling NIP-46 request: method={}, id={}", request.method, request.id);
        match request.method.as_str() {
            "connect" => self.handle_connect(request, client_pubkey).await,
            "sign_event" => self.handle_sign_event(request, client_pubkey).await,
            "get_public_key" => self.handle_get_public_key(request).await,
            "ping" => self.handle_ping(request).await,
            _ => Nip46Response {
                id: request.id,
                result: None,
                error: Some(format!("Unknown method: {}", request.method)),
            },
        }
    }
    
    /// connect method - authorize client
    async fn handle_connect(
        &self,
        request: Nip46Request,
        client_pubkey: PublicKey,
    ) -> Nip46Response {
        // Parse params: [remote_signer_pubkey, optional_secret, optional_perms]
        if request.params.is_empty() {
            return Nip46Response {
                id: request.id,
                result: None,
                error: Some("Missing remote pubkey".to_string()),
            };
        }

        // For MVP, we'll just authorize the client. 
        // In a real scenario, we might verify the remote_signer_pubkey or a secret.
        // If a secret is provided, we should verify it.
        if request.params.len() > 1 {
            let secret = &request.params[1];
            if !secret.is_empty() {
                match self.db.get_config("bunker_secret") {
                    Ok(Some(expected_secret)) => {
                        if secret != &expected_secret {
                            warn!("Invalid secret provided by {}", client_pubkey);
                            return Nip46Response {
                                id: request.id,
                                result: None,
                                error: Some("Invalid secret".to_string()),
                            };
                        }
                    }
                    Ok(None) => {
                        // No secret configured, allow anyway or reject?
                        // Let's allow for now as per "Working > Perfect"
                    }
                    Err(e) => {
                        return Nip46Response {
                            id: request.id,
                            result: None,
                            error: Some(format!("Database error: {}", e)),
                        };
                    }
                }
            }
        }

        let mut connections = self.connections.write().await;
        connections.insert(client_pubkey, ClientPermissions {
            allowed_methods: vec!["*".to_string()], // Grant all for now, or use params[2] if provided
            connected_at: Utc::now(),
        });

        Nip46Response {
            id: request.id,
            result: Some("ack".to_string()),
            error: None,
        }
    }
    
    /// sign_event method - sign a Nostr event
    async fn handle_sign_event(
        &self,
        request: Nip46Request,
        client_pubkey: PublicKey,
    ) -> Nip46Response {
        if request.params.is_empty() {
            return Nip46Response {
                id: request.id,
                result: None,
                error: Some("Missing event to sign".to_string()),
            };
        }

        let unsigned_event_json = &request.params[0];
        let unsigned_event: UnsignedEvent = match serde_json::from_str(unsigned_event_json) {
            Ok(ev) => ev,
            Err(e) => return Nip46Response {
                id: request.id,
                result: None,
                error: Some(format!("Invalid event JSON: {}", e)),
            },
        };

        // Check permission
        if !self.check_permission(client_pubkey, "sign_event", Some(unsigned_event.kind.as_u16() as u32)).await {
            return Nip46Response {
                id: request.id,
                result: None,
                error: Some("Forbidden".to_string()),
            };
        }

        // Sign with bunker keys
        match self.signer.read().await.sign_event(unsigned_event.clone()).await {
            Ok(signed_event) => {
                // Log to database
                if let Err(e) = self.db.log_signing_event(
                    &signed_event.id.to_hex(),
                    &client_pubkey.to_hex(),
                    signed_event.kind.as_u16() as u32,
                    Utc::now(),
                ) {
                    warn!("Failed to log signing event: {}", e);
                }

                Nip46Response {
                    id: request.id,
                    result: Some(signed_event.as_json()),
                    error: None,
                }
            }
            Err(e) => Nip46Response {
                id: request.id,
                result: None,
                error: Some(format!("Signing error: {}", e)),
            },
        }
    }
    
    /// get_public_key method - return user pubkey
    async fn handle_get_public_key(&self, request: Nip46Request) -> Nip46Response {
        let pubkey = self.signer.read().await.public_key_bech32();
        Nip46Response {
            id: request.id,
            result: Some(pubkey),
            error: None,
        }
    }
    
    /// ping method - health check
    async fn handle_ping(&self, request: Nip46Request) -> Nip46Response {
        Nip46Response {
            id: request.id,
            result: Some("pong".to_string()),
            error: None,
        }
    }

    async fn check_permission(
        &self,
        client_pubkey: PublicKey,
        method: &str,
        event_kind: Option<u32>,
    ) -> bool {
        let connections = self.connections.read().await;
        
        if let Some(perms) = connections.get(&client_pubkey) {
            if perms.allowed_methods.contains(&"*".to_string()) {
                return true;
            }
            
            if let Some(kind) = event_kind {
                let required = format!("{}:{}", method, kind);
                perms.allowed_methods.contains(&required)
            } else {
                perms.allowed_methods.contains(&method.to_string())
            }
        } else {
            false
        }
    }
}
