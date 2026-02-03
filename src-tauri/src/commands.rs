use crate::config::AppConfig;
use crate::installer::ClaudeInstaller;
use crate::models::*;
use crate::AppState;
use tauri::{command, State};

#[command]
pub async fn check_claude_installed() -> Result<InstallationStatus, String> {
    Ok(ClaudeInstaller::check_installation())
}

#[command]
pub async fn install_node() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        ClaudeInstaller::install_npm_windows()
    }
    #[cfg(not(target_os = "windows"))]
    {
        ClaudeInstaller::install_npm_unix()
    }
}

#[command]
pub async fn install_claude() -> Result<String, String> {
    ClaudeInstaller::install()
}

#[command]
pub async fn get_instances(
    state: State<'_, AppState>,
    active_only: bool,
) -> Result<Vec<ClaudeInstance>, String> {
    let db = state.db.lock().await;
    db.get_instances(active_only)
        .map_err(|e| format!("Database error: {}", e))
}

#[command]
pub async fn get_instance_details(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<ClaudeInstance>, String> {
    let db = state.db.lock().await;
    let instances = db.get_instances(false).map_err(|e| e.to_string())?;
    Ok(instances.into_iter().find(|i| i.id == id))
}

#[command]
pub async fn get_instance_resources(
    state: State<'_, AppState>,
    instance_id: String,
    limit: i64,
) -> Result<Vec<InstanceResource>, String> {
    let db = state.db.lock().await;
    db.get_instance_resources(&instance_id, limit)
        .map_err(|e| format!("Database error: {}", e))
}

#[command]
pub async fn get_instance_sessions(
    state: State<'_, AppState>,
    instance_id: String,
) -> Result<Vec<SessionEvent>, String> {
    let db = state.db.lock().await;
    db.get_instance_sessions(&instance_id)
        .map_err(|e| format!("Database error: {}", e))
}

#[command]
pub async fn kill_instance(
    state: State<'_, AppState>,
    pid: u32,
) -> Result<(), String> {
    let mut monitor = state.monitor.lock().await;
    monitor.kill_process(pid)
}

#[command]
pub async fn get_hook_script(port: u16) -> Result<String, String> {
    let script = format!(
        r#"#!/bin/bash
# Claude Code Monitor Hook Script
# This script sends events to the monitor at http://localhost:{port}

HOOK_URL="http://localhost:{port}/hook"
PID=$$
CWD="$(pwd)"

send_hook() {{
    local event_type="$1"
    local data="$2"

    curl -s -X POST "$HOOK_URL" \
        -H "Content-Type: application/json" \
        -d "{{\"
            event\": \"$event_type\", \"
            pid\": $PID, \"
            cwd\": \"$CWD\", \"
            timestamp\": $(date +%s), \"
            data\": $data
        }}" > /dev/null 2>&1 || true
}}

# Hook into various events
case "$1" in
    start)
        send_hook "task_start" "null"
        ;;
    end)
        send_hook "task_end" "null"
        ;;
    prompt)
        send_hook "prompt" "{{\\"content\\": \\"$2\\"}}"
        ;;
    response)
        send_hook "response" "{{\\"content\\": \\"$2\\"}}"
        ;;
    *)
        send_hook "$1" "null"
        ;;
esac
"#
    );
    Ok(script)
}

#[command]
pub async fn install_hook(
    state: State<'_, AppState>,
) -> Result<String, String> {
    // 获取 hook 服务端口
    let hook_server = state.hook_server.lock().await;
    let port = 9876; // 默认端口
    drop(hook_server);

    // 获取 hook 脚本内容
    let script = get_hook_script(port).await?;

    // 保存到用户目录
    let hook_dir = dirs::home_dir()
        .ok_or("Cannot find home directory")?
        .join(".claude-monitor");

    std::fs::create_dir_all(&hook_dir).map_err(|e| e.to_string())?;

    let hook_path = hook_dir.join("hook.sh");
    std::fs::write(&hook_path, script).map_err(|e| e.to_string())?;

    // 设置可执行权限（Unix）
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&hook_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms).map_err(|e| e.to_string())?;
    }

    // 创建 wrapper 脚本
    let wrapper = format!(
        r#"#!/bin/bash
# Claude Code wrapper with hook support
export CLAUDE_CODE_HOOKS="{}"
claude "$@"
"#,
        hook_path.to_string_lossy()
    );

    let wrapper_path = hook_dir.join("claude-with-hook");
    std::fs::write(&wrapper_path, wrapper).map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&wrapper_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&wrapper_path, perms).map_err(|e| e.to_string())?;
    }

    Ok(format!(
        "Hook installed successfully.\nHook script: {}\nWrapper: {}",
        hook_path.display(),
        wrapper_path.display()
    ))
}

#[command]
pub async fn uninstall_hook() -> Result<String, String> {
    let hook_dir = dirs::home_dir()
        .ok_or("Cannot find home directory")?
        .join(".claude-monitor");

    if hook_dir.exists() {
        std::fs::remove_dir_all(&hook_dir).map_err(|e| e.to_string())?;
        Ok("Hook uninstalled successfully".to_string())
    } else {
        Ok("Hook not installed".to_string())
    }
}

#[command]
pub async fn get_workspace_stats(
    state: State<'_, AppState>,
) -> Result<Vec<WorkspaceStats>, String> {
    let db = state.db.lock().await;
    db.get_workspace_stats()
        .map_err(|e| format!("Database error: {}", e))
}

#[command]
pub async fn search_history(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<SessionEvent>, String> {
    let db = state.db.lock().await;
    db.search_history(&query)
        .map_err(|e| format!("Database error: {}", e))
}

#[command]
pub fn get_config() -> AppConfig {
    AppConfig::load()
}

#[command]
pub fn save_config(config: AppConfig) -> Result<(), String> {
    config.save()
}

#[command]
pub fn is_hook_installed() -> bool {
    AppConfig::is_hook_installed()
}
