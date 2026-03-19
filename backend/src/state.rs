use std::sync::Arc;
use tokio::sync::RwLock;
use crate::signer::Signer;
use crate::db::Database;
use crate::nip46::Nip46Handler;
use crate::config::Config;

use std::sync::atomic::{AtomicU64, Ordering};

/// Shared application state for both HTTP and NIP-46
#[derive(Clone)]
pub struct AppState {
    pub signer: Arc<RwLock<Signer>>,
    pub db: Arc<Database>,
    pub nip46_handler: Arc<Nip46Handler>,
    pub config: Arc<RwLock<Config>>,
    http_request_count: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(signer: Signer, db: Database, config: Config) -> Self {
        let signer = Arc::new(RwLock::new(signer));
        let config = Arc::new(RwLock::new(config));
        let db = Arc::new(db);
        let nip46_handler = Arc::new(Nip46Handler::new(signer.clone(), (*db).clone()));
        
        Self {
            signer,
            db,
            nip46_handler,
            config,
            http_request_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn increment_http_request_count(&self) {
        self.http_request_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn http_request_count(&self) -> u64 {
        self.http_request_count.load(Ordering::SeqCst)
    }
}
