use std::sync::Arc;
use tokio::sync::RwLock;
use bunker::nip46::{Nip46Handler, Nip46Request};
use bunker::signer::Signer;
use bunker::db::Database;
use nostr::prelude::*;
use tempfile::tempdir;

#[tokio::test]
async fn test_nip46_ping() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());
    let handler = Nip46Handler::new(Arc::new(RwLock::new(signer)), db);

    let request = Nip46Request {
        id: "test_ping".to_string(),
        method: "ping".to_string(),
        params: vec![],
    };

    let response = handler.handle_request(request, keys.public_key()).await;
    assert_eq!(response.id, "test_ping");
    assert_eq!(response.result, Some("pong".to_string()));
    assert_eq!(response.error, None);
}

#[tokio::test]
async fn test_nip46_get_public_key() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());
    let handler = Nip46Handler::new(Arc::new(RwLock::new(signer.clone())), db);

    let request = Nip46Request {
        id: "test_pubkey".to_string(),
        method: "get_public_key".to_string(),
        params: vec![],
    };

    let response = handler.handle_request(request, keys.public_key()).await;
    assert_eq!(response.id, "test_pubkey");
    assert_eq!(response.result, Some(signer.public_key_bech32()));
}

#[tokio::test]
async fn test_nip46_connect_and_sign() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());
    let handler = Nip46Handler::new(Arc::new(RwLock::new(signer)), db);

    let client_keys = Keys::generate();
    let client_pubkey = client_keys.public_key();

    // 1. Connect
    let connect_req = Nip46Request {
        id: "req1".to_string(),
        method: "connect".to_string(),
        params: vec![client_pubkey.to_hex()],
    };
    let connect_res = handler.handle_request(connect_req, client_pubkey).await;
    assert_eq!(connect_res.result, Some("ack".to_string()));

    // 2. Sign Event
    let unsigned_event = EventBuilder::new(Kind::TextNote, "hello").build(client_pubkey);
    let sign_req = Nip46Request {
        id: "req2".to_string(),
        method: "sign_event".to_string(),
        params: vec![serde_json::to_string(&unsigned_event).unwrap()],
    };

    let sign_res = handler.handle_request(sign_req, client_pubkey).await;
    if let Some(ref err) = sign_res.error {
        panic!("Sign error: {}", err);
    }
    assert!(sign_res.result.is_some());
    let signed_event: Event = serde_json::from_str(&sign_res.result.unwrap()).unwrap();
    assert_eq!(signed_event.kind, Kind::TextNote);
    assert_eq!(signed_event.content, "hello");
    // Should be signed by bunker keys, not client keys
    assert_eq!(signed_event.pubkey, keys.public_key());
}

#[tokio::test]
async fn test_nip46_unauthorized() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());
    let handler = Nip46Handler::new(Arc::new(RwLock::new(signer)), db);

    let client_pubkey = Keys::generate().public_key();

    // Sign Event without connecting first
    let unsigned_event = EventBuilder::new(Kind::TextNote, "hello").build(client_pubkey);
    let sign_req = Nip46Request {
        id: "req1".to_string(),
        method: "sign_event".to_string(),
        params: vec![serde_json::to_string(&unsigned_event).unwrap()],
    };

    let sign_res = handler.handle_request(sign_req, client_pubkey).await;
    assert_eq!(sign_res.error, Some("Forbidden".to_string()));
}
