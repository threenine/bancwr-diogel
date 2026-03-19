use bunker::db::Database;
use chrono::Utc;

#[test]
fn test_db_init_and_tables() {
    let _db = Database::new(":memory:").expect("Failed to create in-memory database");
}

#[test]
fn test_db_log_signing_event() {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    let event_id = "test_event_id";
    let pubkey = "test_pubkey";
    let kind = 1;
    let now = Utc::now();

    db.log_signing_event(event_id, pubkey, kind, now).expect("Failed to log event");

    let logs = db.get_recent_logs(10).expect("Failed to get logs");
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].event_id, event_id);
    assert_eq!(logs[0].pubkey, pubkey);
    assert_eq!(logs[0].event_kind, kind);
}

#[test]
fn test_db_config_storage() {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    let key = "test_key";
    let value = "test_value";

    db.set_config(key, value).expect("Failed to set config");
    let retrieved = db.get_config(key).expect("Failed to get config");
    assert_eq!(retrieved, Some(value.to_string()));

    // Test update
    let new_value = "new_value";
    db.set_config(key, new_value).expect("Failed to update config");
    let retrieved = db.get_config(key).expect("Failed to get config");
    assert_eq!(retrieved, Some(new_value.to_string()));
}

#[test]
fn test_db_get_config_non_existent() {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    let retrieved = db.get_config("non_existent").expect("Failed to get config");
    assert_eq!(retrieved, None);
}

#[test]
fn test_db_recent_logs_ordering() {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    let now = Utc::now();
    
    db.log_signing_event("event1", "pub1", 1, now).unwrap();
    db.log_signing_event("event2", "pub2", 2, now + chrono::Duration::seconds(1)).unwrap();

    let logs = db.get_recent_logs(10).unwrap();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].event_id, "event2");
    assert_eq!(logs[1].event_id, "event1");
}

#[test]
fn test_db_team_management() {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    
    // Add member
    let id = db.add_team_member("Alice", "npub1...", "admin").expect("Failed to add member");
    
    // Get members
    let members = db.get_team_members().expect("Failed to get members");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].name, "Alice");
    assert_eq!(members[0].pubkey, "npub1...");
    assert_eq!(members[0].role, "admin");
    assert_eq!(members[0].id, id);
    
    // Remove member
    db.remove_team_member(id).expect("Failed to remove member");
    let members = db.get_team_members().expect("Failed to get members");
    assert_eq!(members.len(), 0);
}
