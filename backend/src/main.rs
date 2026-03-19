use bunker::config::Config;
use bunker::db::Database;
use bunker::relay::RelayClient;
use bunker::server::{create_router, shutdown_signal};
use bunker::state::AppState;
use bunker::signer::Signer;
use std::net::SocketAddr;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    info!("Bancwr Diogel starting...");

    let config = Config::load()?;
    let db = Database::new(&config.db_path)?;

    let signer = Signer::new(config.secret_key.clone());
    info!("Nsec loaded. Public key: {}", signer.public_key_bech32());

    let state = AppState::new(signer.clone(), db, config.clone());

    let port = config.port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("HTTP API listening on http://{}", addr);

    let http_state = state.clone();
    let http_task = tokio::spawn(async move {
        let app = create_router(http_state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| anyhow::anyhow!("HTTP server error: {}", e))
    });

    // Start NIP-46 relay client (if enabled)
    let relay_task = if config.nip46_enabled {
        let relay_state = state.clone();
        Some(tokio::spawn(async move {
            info!("Starting NIP-46 relay client...");
            let client = RelayClient::new(
                config.relay_urls,
                relay_state,
            ).await?;
            client.run().await
        }))
    } else {
        None
    };

    // Wait for shutdown signal or tasks
    tokio::select! {
        result = http_task => {
            if let Ok(Err(e)) = result {
                error!("HTTP server error: {}", e);
            }
        }
        result = async {
            if let Some(task) = relay_task {
                task.await
            } else {
                std::future::pending().await
            }
        } => {
            if let Ok(Err(e)) = result {
                error!("Relay client error: {}", e);
            }
        }
        _ = shutdown_signal() => {
            info!("Shutdown signal received");
        }
    }

    Ok(())
}
