import { InstallationStatus } from '../types'

interface InstallProps {
  installStatus: InstallationStatus | null
  installingNode: boolean
  installingClaude: boolean
  onInstallNode: () => Promise<void>
  onInstallClaude: () => Promise<void>
  onRefresh: () => void
}

export function Install({
  installStatus,
  installingNode,
  installingClaude,
  onInstallNode,
  onInstallClaude,
  onRefresh
}: InstallProps) {
  const isInstalled = installStatus?.installed

  const handleInstallClaude = async () => {
    try {
      await onInstallClaude()
      alert('Claude Code 安装成功')
      onRefresh()
    } catch (e) {
      alert('安装失败: ' + e)
    }
  }

  const handleInstallNode = async () => {
    try {
      const result = await onInstallNode()
      alert(result)
    } catch (e) {
      alert('安装失败: ' + e)
    }
  }

  return (
    <>
      <div className="main-header">
        <h2>安装管理</h2>
      </div>
      <div className="content-scroll">
      <div className="install-section">
        <div className="install-status">
          <div className={`install-status-icon ${isInstalled ? 'ok' : 'warning'}`}>
            {isInstalled ? '✓' : '!'}
          </div>
          <div className="install-status-info">
            <h3>{isInstalled ? 'Claude Code 已安装' : 'Claude Code 未安装'}</h3>
            <p>
              {isInstalled
                ? `版本: ${installStatus?.version || '未知'}${installStatus?.path ? ` | 路径: ${installStatus.path}` : ''}`
                : '点击下方按钮自动安装'}
            </p>
          </div>
        </div>

        {!isInstalled && (
          <div className="install-steps">
            <div className="install-step">
              <div className="step-number">1</div>
              <div className="step-content">
                <h4>安装 Node.js</h4>
                <p>Claude Code 需要 Node.js 环境 (v18+)</p>
                <button
                  className="btn btn-primary"
                  onClick={handleInstallNode}
                  disabled={installingNode}
                >
                  {installingNode ? '安装中...' : '自动安装 Node.js'}
                </button>
              </div>
            </div>
            <div className="install-step">
              <div className="step-number">2</div>
              <div className="step-content">
                <h4>安装 Claude Code</h4>
                <p>通过 npm 全局安装 Claude Code CLI</p>
                <div className="code-block">npm install -g @anthropic-ai/claude-code</div>
                <button
                  className="btn btn-primary"
                  onClick={handleInstallClaude}
                  disabled={installingClaude}
                  style={{ marginTop: '12px' }}
                >
                  {installingClaude ? '安装中...' : '自动安装 Claude Code'}
                </button>
              </div>
            </div>
          </div>
        )}

        {isInstalled && (
          <div className="card">
            <div className="card-header">
              <span className="card-title">卸载</span>
            </div>
            <p style={{ color: 'var(--text-secondary)', fontSize: '13px', marginBottom: '12px' }}>
              如需卸载 Claude Code，请在终端运行：
            </p>
            <div className="code-block">npm uninstall -g @anthropic-ai/claude-code</div>
          </div>
        )}
      </div>
      </div>
    </>
  )
}
