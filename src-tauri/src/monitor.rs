use crate::models::ClaudeInstance;
use chrono::Local;
use std::collections::HashMap;
use sysinfo::{Process, System};
use uuid::Uuid;

pub struct ProcessMonitor {
    system: System,
    instance_map: HashMap<u32, String>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            instance_map: HashMap::new(),
        }
    }

    pub async fn scan_instances(&mut self) -> Vec<ClaudeInstance> {
        self.system.refresh_all();

        let mut instances = Vec::new();

        for (pid, process) in self.system.processes() {
            let name = process.name().to_lowercase();
            let cmdline = process.cmd().join(" ");

            // 检测 claude 进程
            if self.is_claude_process(&name, &cmdline) {
                let pid_u32 = pid.as_u32();

                // 获取或生成实例 ID
                let instance_id = self
                    .instance_map
                    .entry(pid_u32)
                    .or_insert_with(|| Uuid::new_v4().to_string())
                    .clone();

                let cwd = process.cwd().map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                let start_time = process.start_time();
                let start_datetime = chrono::DateTime::from_timestamp(start_time as i64, 0)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|| Local::now());

                let cpu_usage = process.cpu_usage();
                let memory_mb = (process.memory() as f64) / 1024.0 / 1024.0;

                instances.push(ClaudeInstance {
                    id: instance_id,
                    pid: pid_u32,
                    cwd,
                    cmdline: cmdline.clone(),
                    status: "running".to_string(),
                    start_time: start_datetime,
                    last_seen: Local::now(),
                    cpu_percent: cpu_usage,
                    memory_mb,
                });
            }
        }

        // 清理已不存在的进程
        let active_pids: Vec<u32> = instances.iter().map(|i| i.pid).collect();
        self.instance_map
            .retain(|pid, _| active_pids.contains(pid));

        instances
    }

    fn is_claude_process(&self, name: &str, cmdline: &str) -> bool {
        // Windows: node.exe running claude
        // macOS/Linux: claude or node with claude
        let indicators = [
            "claude",
            "@anthropic-ai/claude-code",
            "claude-code",
        ];

        let is_node = name.contains("node") || name.contains("npm");
        let is_claude = name.contains("claude");

        if is_claude {
            return true;
        }

        if is_node {
            return indicators.iter().any(|ind| cmdline.to_lowercase().contains(ind));
        }

        false
    }

    pub fn kill_process(&mut self, pid: u32) -> Result<(), String> {
        self.system.refresh_all();

        if let Some(process) = self.system.process((pid as usize).into()) {
            if process.kill() {
                self.instance_map.remove(&pid);
                Ok(())
            } else {
                Err("Failed to kill process".to_string())
            }
        } else {
            Err("Process not found".to_string())
        }
    }
}
