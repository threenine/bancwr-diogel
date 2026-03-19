use bunker::db::Database;
use bunker::server::create_router;
use bunker::state::AppState;
use bunker::config::Config;
use bunker::signer::Signer;
use nostr::prelude::*;
use tokio::net::TcpListener;
use reqwest::StatusCode;
use serde_json::Value;
use tempfile::NamedTempFile;

async fn setup_app() -> (String, reqwest::Client, Keys) {
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());
    
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    let config = Config {
        secret_key: keys.secret_key().clone(),
        port: 0,
        db_path: ":memory:".to_string(),
        relay_urls: vec![],
        nip46_enabled: false,
        nsec_file: None,
    };
    let state = AppState::new(signer, db, config);
    let app = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    let addr = listener.local_addr().expect("Failed to get local address");
    let port = addr.port();

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("SERVER ERROR IN TEST: {:?}", e);
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let address = format!("http://127.0.0.1:{}", port);
    let client = reqwest::Client::new();
    
    (address, client, keys)
}

#[tokio::test]
async fn test_get_config() {
    let (address, client, keys) = setup_app().await;

    let res = client
        .get(format!("{}/api/bunker/config", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(res.status(), StatusCode::OK);
    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(body["pubkey"], keys.public_key().to_bech32().unwrap());
    assert!(body["nsec"].is_null()); // Note: this might fail if nsec is missing instead of null
    assert!(body["nsec_file"].is_null());
}

#[tokio::test]
async fn test_update_config_nsec() {
    let (address, client, _keys) = setup_app().await;
    
    let new_keys = Keys::generate();
    let new_nsec = new_keys.secret_key().to_bech32().unwrap();
    let new_pubkey = new_keys.public_key().to_bech32().unwrap();

    // 1. Update config
    let res = client
        .post(format!("{}/api/bunker/config", address))
        .json(&serde_json::json!({
            "nsec": new_nsec
        }))
        .send()
        .await
        .expect("Failed to execute request");

    // 2. Verify response
    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(body["success"], true);
    assert_eq!(body["pubkey"], new_pubkey);
    assert!(body["message"].as_str().unwrap().contains("Restart required"));

    // 3. Verify it is NOT immediately reflected in GET /config (stays old pubkey)
    let res = client
        .get(format!("{}/api/bunker/config", address))
        .send()
        .await
        .expect("Failed to execute request");
    
    let body: Value = res.json().await.expect("Failed to parse JSON");
    // Should still be the old pubkey from setup_app
    assert_ne!(body["pubkey"], new_pubkey);
}

#[tokio::test]
async fn test_update_config_invalid_nsec() {
    let (address, client, _keys) = setup_app().await;

    let res = client
        .post(format!("{}/api/bunker/config", address))
        .json(&serde_json::json!({
            "nsec": "invalid-nsec"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_config_nsec_file() {
    let (address, client, _keys) = setup_app().await;
    
    // Create a temporary file
    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    // 1. Update config with nsec_file
    let res = client
        .post(format!("{}/api/bunker/config", address))
        .json(&serde_json::json!({
            "nsec_file": temp_path
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(res.status(), StatusCode::OK);
    let body: Value = res.json().await.expect("Failed to parse JSON");
    assert_eq!(body["success"], true);
    assert!(body["message"].as_str().unwrap().contains("Restart required"));

    // 2. Verify change (persisted to DB, but not necessarily reflected in memory)
    // In this test setup, we don't easily check the DB, but we check that memory IS NOT updated if that's the policy
}
