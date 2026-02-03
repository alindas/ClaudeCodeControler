import { AppConfig } from '../types'

interface HooksProps {
  config: AppConfig
  installing: boolean
  onToggle: () => Promise<void>
}

export function Hooks({ config, installing, onToggle }: HooksProps) {
  const handleToggle = async () => {
    try {
      await onToggle()
    } catch (e) {
      alert('操作失败: ' + e)
    }
  }

  return (
    <>
      <div className="main-header">
        <h2>Hook 配置</h2>
      </div>
      <div className="content-scroll">
      <div className="install-section">
        <div className="card">
          <div className="card-header">
            <span className="card-title">自动 Hook 注入</span>
          </div>
          <p style={{ color: 'var(--text-secondary)', fontSize: '13px', marginBottom: '16px' }}>
            启用后，工具会自动生成 Hook 脚本，捕获 Claude Code 的事件和会话内容。
          </p>
          <div className="setting-item">
            <div>
              <div className="setting-label">启用 Hook</div>
              <div className="setting-description">捕获任务开始/结束、提示词、响应等事件</div>
            </div>
            <button
              className={`toggle ${config.hook_enabled ? 'active' : ''}`}
              onClick={handleToggle}
              disabled={installing}
              style={{
                border: 'none',
                background: config.hook_enabled ? 'var(--accent)' : 'var(--bg-tertiary)'
              }}
            >
              <div className="toggle-handle"></div>
            </button>
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <span className="card-title">手动使用 Hook</span>
          </div>
          <p style={{ color: 'var(--text-secondary)', fontSize: '13px', marginBottom: '12px' }}>
            如果不想全局启用，可以手动使用带 Hook 的 wrapper：
          </p>
          <div className="code-block">{`# 使用 wrapper 启动
cd ~/.claude-monitor
./claude-with-hook

# 或设置环境变量
export CLAUDE_CODE_HOOKS=~/.claude-monitor/hook.sh
claude`}</div>
        </div>

        <div className="card">
          <div className="card-header">
            <span className="card-title">Hook 安装位置</span>
          </div>
          <div className="code-block" style={{ marginBottom: '12px' }}>
            {`~/.claude-monitor/
  ├── hook.sh          # Hook 脚本
  └── claude-with-hook # Wrapper 脚本`}
          </div>
        </div>
      </div>
      </div>
    </>
  )
}
