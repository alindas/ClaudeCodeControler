use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeInstance {
    pub id: String,
    pub pid: u32,
    pub cwd: String,
    pub cmdline: String,
    pub status: String,
    pub start_time: DateTime<Local>,
    pub last_seen: DateTime<Local>,
    pub cpu_percent: f32,
    pub memory_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceResource {
    pub instance_id: String,
    pub timestamp: DateTime<Local>,
    pub cpu_percent: f32,
    pub memory_mb: f64,
    pub disk_read_mb: u64,
    pub disk_write_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub id: String,
    pub instance_id: String,
    pub event_type: String,
    pub content: String,
    pub timestamp: DateTime<Local>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStats {
    pub path: String,
    pub name: String,
    pub session_count: i64,
    pub total_tokens: i64,
    pub last_active: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEvent {
    pub event: String,
    pub pid: u32,
    pub cwd: String,
    pub timestamp: i64,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}
