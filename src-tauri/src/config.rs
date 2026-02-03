use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub hook_enabled: bool,
    #[serde(default)]
    pub auto_start_monitor: bool,
    #[serde(default)]
    pub polling_interval_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hook_enabled: false,
            auto_start_monitor: true,
            polling_interval_secs: 5,
        }
    }
}

impl AppConfig {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("claude-code-monitor").join("config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Cannot get config directory")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn is_hook_installed() -> bool {
        // 检查 hook 脚本是否存在
        if let Some(home) = dirs::home_dir() {
            let hook_path = home.join(".claude-monitor").join("hook.sh");
            return hook_path.exists();
        }
        false
    }
}
