use duckdb::{params, Connection};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::path::Path;
use std::fs;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone)]
pub struct SigningLog {
    pub id: Uuid,
    pub event_id: String,
    pub pubkey: String,
    pub event_kind: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub pubkey: String,
    pub role: String, // "admin", "signer", "viewer"
    pub created_at: DateTime<Utc>,
}

impl Database {
    /// Initialize database connection and create tables if not exist
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let path = Path::new(db_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let conn = Connection::open(db_path)?;
        
        // Create tables
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS signing_logs (
                id UUID PRIMARY KEY,
                event_id TEXT NOT NULL,
                pubkey TEXT NOT NULL,
                event_kind INTEGER NOT NULL,
                timestamp TIMESTAMP NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON signing_logs(timestamp DESC);
            
            CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS team_members (
                id UUID PRIMARY KEY,
                name TEXT NOT NULL,
                pubkey TEXT NOT NULL UNIQUE,
                role TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL
            );"
        )?;

        info!("Database initialized at {}", db_path);
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }
    
    /// Log a signing event
    pub fn log_signing_event(
        &self,
        event_id: &str,
        pubkey: &str,
        event_kind: u32,
        timestamp: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let id = Uuid::new_v4();
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO signing_logs (id, event_id, pubkey, event_kind, timestamp) VALUES (?, ?, ?, ?, ?)",
            params![id, event_id, pubkey, event_kind, timestamp],
        )?;
        Ok(())
    }
    
    /// Get recent signing logs (for API)
    pub fn get_recent_logs(&self, limit: usize) -> anyhow::Result<Vec<SigningLog>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, event_id, pubkey, event_kind, timestamp FROM signing_logs ORDER BY timestamp DESC LIMIT ?"
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(SigningLog {
                id: row.get(0)?,
                event_id: row.get(1)?,
                pubkey: row.get(2)?,
                event_kind: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row?);
        }
        Ok(logs)
    }
    
    /// Store configuration
    pub fn set_config(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let now = Utc::now();
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO config (key, value, updated_at) VALUES (?, ?, ?) 
             ON CONFLICT(key) DO UPDATE SET value = EXCLUDED.value, updated_at = EXCLUDED.updated_at",
            params![key, value, now],
        )?;
        Ok(())
    }
    
    /// Get configuration
    pub fn get_config(&self, key: &str) -> anyhow::Result<Option<String>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?")?;
        let mut rows = stmt.query(params![key])?;
        
        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Get total number of signatures
    pub fn signature_count(&self) -> anyhow::Result<u64> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let count: u64 = conn.query_row("SELECT count(*) FROM signing_logs", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Add a team member
    pub fn add_team_member(
        &self,
        name: &str,
        pubkey: &str,
        role: &str,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO team_members (id, name, pubkey, role, created_at) VALUES (?, ?, ?, ?, ?)",
            params![id, name, pubkey, role, Utc::now()],
        )?;
        Ok(id)
    }

    /// Get all team members
    pub fn get_team_members(&self) -> anyhow::Result<Vec<TeamMember>> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, pubkey, role, created_at FROM team_members ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![], |row| {
            Ok(TeamMember {
                id: row.get(0)?,
                name: row.get(1)?,
                pubkey: row.get(2)?,
                role: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut members = Vec::new();
        for row in rows {
            members.push(row?);
        }
        Ok(members)
    }

    /// Remove a team member
    pub fn remove_team_member(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM team_members WHERE id = ?",
            params![id],
        )?;
        Ok(())
    }
}
