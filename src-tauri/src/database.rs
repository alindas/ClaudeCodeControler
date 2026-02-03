use crate::models::*;
use chrono::{DateTime, Local};
use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    fn get_db_path() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| rusqlite::Error::InvalidPath(PathBuf::from("data_dir not found")))?;
        let app_dir = data_dir.join("claude-code-monitor");
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| rusqlite::Error::InvalidPath(PathBuf::from(e.to_string())))?;
        Ok(app_dir.join("data.db"))
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS instances (
                id TEXT PRIMARY KEY,
                pid INTEGER NOT NULL,
                cwd TEXT NOT NULL,
                cmdline TEXT,
                status TEXT NOT NULL,
                start_time INTEGER NOT NULL,
                last_seen INTEGER NOT NULL,
                cpu_percent REAL DEFAULT 0,
                memory_mb REAL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS resources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                instance_id TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                cpu_percent REAL NOT NULL,
                memory_mb REAL NOT NULL,
                disk_read_mb INTEGER DEFAULT 0,
                disk_write_mb INTEGER DEFAULT 0,
                FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                instance_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                content TEXT,
                timestamp INTEGER NOT NULL,
                metadata TEXT,
                FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS workspaces (
                path TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                session_count INTEGER DEFAULT 0,
                total_tokens INTEGER DEFAULT 0,
                last_active INTEGER
            );

            CREATE INDEX IF NOT EXISTS idx_instances_pid ON instances(pid);
            CREATE INDEX IF NOT EXISTS idx_resources_instance ON resources(instance_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_instance ON sessions(instance_id);
            CREATE INDEX IF NOT EXISTS idx_resources_timestamp ON resources(timestamp);
            "
        )?;
        Ok(())
    }

    pub fn upsert_instance(&self, instance: &ClaudeInstance) -> Result<()> {
        self.conn.execute(
            "INSERT INTO instances (id, pid, cwd, cmdline, status, start_time, last_seen, cpu_percent, memory_mb)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
             status = excluded.status,
             last_seen = excluded.last_seen,
             cpu_percent = excluded.cpu_percent,
             memory_mb = excluded.memory_mb",
            [
                &instance.id,
                &instance.pid.to_string(),
                &instance.cwd,
                instance.cmdline.as_str(),
                &instance.status,
                &instance.start_time.timestamp().to_string(),
                &instance.last_seen.timestamp().to_string(),
                &instance.cpu_percent.to_string(),
                &instance.memory_mb.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn get_instances(&self, active_only: bool) -> Result<Vec<ClaudeInstance>> {
        let sql = if active_only {
            "SELECT id, pid, cwd, cmdline, status, start_time, last_seen, cpu_percent, memory_mb
             FROM instances WHERE status = 'running' ORDER BY last_seen DESC"
        } else {
            "SELECT id, pid, cwd, cmdline, status, start_time, last_seen, cpu_percent, memory_mb
             FROM instances ORDER BY last_seen DESC LIMIT 100"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let start_ts: i64 = row.get(5)?;
            let last_ts: i64 = row.get(6)?;
            Ok(ClaudeInstance {
                id: row.get(0)?,
                pid: row.get(1)?,
                cwd: row.get(2)?,
                cmdline: row.get(3)?,
                status: row.get(4)?,
                start_time: DateTime::from_timestamp(start_ts, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now()),
                last_seen: DateTime::from_timestamp(last_ts, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now()),
                cpu_percent: row.get(7)?,
                memory_mb: row.get(8)?,
            })
        })?;

        rows.collect()
    }

    pub fn insert_resource(&self, resource: &InstanceResource) -> Result<()> {
        self.conn.execute(
            "INSERT INTO resources (instance_id, timestamp, cpu_percent, memory_mb, disk_read_mb, disk_write_mb)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &resource.instance_id,
                &resource.timestamp.timestamp().to_string(),
                &resource.cpu_percent.to_string(),
                &resource.memory_mb.to_string(),
                &resource.disk_read_mb.to_string(),
                &resource.disk_write_mb.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn get_instance_resources(&self, instance_id: &str, limit: i64) -> Result<Vec<InstanceResource>> {
        let mut stmt = self.conn.prepare(
            "SELECT instance_id, timestamp, cpu_percent, memory_mb, disk_read_mb, disk_write_mb
             FROM resources WHERE instance_id = ?1 ORDER BY timestamp DESC LIMIT ?2"
        )?;

        let rows = stmt.query_map([instance_id, &limit.to_string()], |row| {
            let ts: i64 = row.get(1)?;
            Ok(InstanceResource {
                instance_id: row.get(0)?,
                timestamp: DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now()),
                cpu_percent: row.get(2)?,
                memory_mb: row.get(3)?,
                disk_read_mb: row.get(4)?,
                disk_write_mb: row.get(5)?,
            })
        })?;

        rows.collect()
    }

    pub fn insert_session_event(&self, event: &SessionEvent) -> Result<()> {
        self.conn.execute(
            "INSERT INTO sessions (id, instance_id, event_type, content, timestamp, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &event.id,
                &event.instance_id,
                &event.event_type,
                event.content.as_str(),
                &event.timestamp.timestamp().to_string(),
                event.metadata.as_deref().unwrap_or(""),
            ],
        )?;
        Ok(())
    }

    pub fn get_instance_sessions(&self, instance_id: &str) -> Result<Vec<SessionEvent>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, instance_id, event_type, content, timestamp, metadata
             FROM sessions WHERE instance_id = ?1 ORDER BY timestamp DESC"
        )?;

        let rows = stmt.query_map([instance_id], |row| {
            let ts: i64 = row.get(4)?;
            Ok(SessionEvent {
                id: row.get(0)?,
                instance_id: row.get(1)?,
                event_type: row.get(2)?,
                content: row.get(3)?,
                timestamp: DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now()),
                metadata: row.get(5)?,
            })
        })?;

        rows.collect()
    }

    pub fn update_instance_status(&self, id: &str, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE instances SET status = ?1 WHERE id = ?2",
            [status, id],
        )?;
        Ok(())
    }

    pub fn get_workspace_stats(&self) -> Result<Vec<WorkspaceStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, name, session_count, total_tokens, last_active
             FROM workspaces ORDER BY last_active DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            let last_ts: Option<i64> = row.get(4)?;
            Ok(WorkspaceStats {
                path: row.get(0)?,
                name: row.get(1)?,
                session_count: row.get(2)?,
                total_tokens: row.get(3)?,
                last_active: last_ts.and_then(|ts| {
                    DateTime::from_timestamp(ts, 0).map(|dt| dt.with_timezone(&Local))
                }),
            })
        })?;

        rows.collect()
    }

    pub fn search_history(&self, query: &str) -> Result<Vec<SessionEvent>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, instance_id, event_type, content, timestamp, metadata
             FROM sessions WHERE content LIKE ?1 OR event_type LIKE ?1
             ORDER BY timestamp DESC LIMIT 50"
        )?;

        let rows = stmt.query_map([&pattern], |row| {
            let ts: i64 = row.get(4)?;
            Ok(SessionEvent {
                id: row.get(0)?,
                instance_id: row.get(1)?,
                event_type: row.get(2)?,
                content: row.get(3)?,
                timestamp: DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now()),
                metadata: row.get(5)?,
            })
        })?;

        rows.collect()
    }
}
