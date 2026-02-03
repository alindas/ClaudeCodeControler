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
pub async fn get_git_hook_script() -> Result<String, String> {
    let script = r#"#!/bin/bash
# Claude Code Git Auto-Commit Hook
# 在每次任务完成后自动提交并推送到 GitHub

HOOK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CWD="$(pwd)"
PID=$$

git_auto_commit() {
    # 检查当前目录是否是 git 仓库
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        echo "[Git Hook] 当前目录不是 Git 仓库，跳过提交"
        return 0
    fi

    # 检查是否有远程仓库
    if ! git remote get-url origin > /dev/null 2>&1; then
        echo "[Git Hook] 未配置远程仓库，跳过推送"
        return 0
    fi

    # 检查是否有变更
    if git diff --quiet HEAD && git diff --staged --quiet; then
        echo "[Git Hook] 没有变更需要提交"
        return 0
    fi

    # 获取当前分支
    BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null || echo "main")

    # 获取任务摘要（从最近的一次提交或环境变量）
    TASK_SUMMARY="${CLAUDE_TASK_SUMMARY:-"自动提交"}"

    echo "[Git Hook] 正在提交更改到 $BRANCH 分支..."

    # 添加所有变更
    git add -A

    # 创建提交
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    COMMIT_MSG="🤖 [$(date '+%H:%M')] $TASK_SUMMARY

自动生成提交
时间: $TIMESTAMP
工作目录: $CWD"

    if git commit -m "$COMMIT_MSG"; then
        echo "[Git Hook] 提交成功"

        # 推送到远程
        echo "[Git Hook] 正在推送到 origin/$BRANCH..."
        if git push origin "$BRANCH"; then
            echo "[Git Hook] ✅ 已成功推送到 origin/$BRANCH"
        else
            echo "[Git Hook] ❌ 推送失败，请检查网络或权限"
        fi
    else
        echo "[Git Hook] ❌ 提交失败"
    fi
}

# Hook into various events
case "$1" in
    start)
        echo "[Git Hook] 任务开始: $CWD"
        ;;
    end)
        echo "[Git Hook] 任务结束，检查是否需要提交..."
        git_auto_commit
        ;;
    *)
        ;;
esac
"#;

    Ok(script.to_string())
}

#[command]
pub async fn install_git_hook() -> Result<String, String> {
    // 获取 hook 脚本内容
    let script = get_git_hook_script().await?;

    // 保存到用户目录
    let hook_dir = dirs::home_dir()
        .ok_or("Cannot find home directory")?
        .join(".claude-monitor");

    std::fs::create_dir_all(&hook_dir).map_err(|e| e.to_string())?;

    let hook_path = hook_dir.join("git-hook.sh");
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

    // 创建 wrapper 脚本（这个 wrapper 会在每次 claude 命令后执行）
    let wrapper = format!(
        r#"#!/bin/bash
# Claude Code wrapper with Git auto-commit hook
SCRIPT_DIR="$(cd "$(dirname "${{BASH_SOURCE[0]}}")" && pwd)"
export CLAUDE_CODE_HOOKS="${{SCRIPT_DIR}}/git-hook.sh"

# 运行 claude
claude "$@"
CLAUDE_EXIT=$?

# 任务结束后执行 git 提交
if [ -f "${{SCRIPT_DIR}}/git-hook.sh" ]; then
    "${{SCRIPT_DIR}}/git-hook.sh" end
fi

exit $CLAUDE_EXIT
"#,
    );

    let wrapper_path = hook_dir.join("claude-with-git-hook");
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
        "Git Auto-Commit Hook 安装成功！\n\n使用方式:\n1. 使用 wrapper 启动 Claude:\n   {}\n\n2. 或在现有项目中手动触发:\n   {} start  # 任务开始\n   {} end    # 任务结束（自动提交）",
        wrapper_path.display(),
        hook_path.display(),
        hook_path.display()
    ))
}

#[command]
pub async fn get_git_hook_status() -> Result<bool, String> {
    let hook_path = dirs::home_dir()
        .ok_or("Cannot find home directory")?
        .join(".claude-monitor")
        .join("git-hook.sh");
    Ok(hook_path.exists())
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
