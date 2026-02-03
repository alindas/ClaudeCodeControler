import { ViewType } from '../types'

interface SidebarProps {
  currentView: ViewType
  onViewChange: (view: ViewType) => void
}

const navItems: { view: ViewType; icon: string; label: string }[] = [
  { view: 'dashboard', icon: '◉', label: '实例监控' },
  { view: 'install', icon: '↓', label: '安装管理' },
  { view: 'hooks', icon: '⚡', label: 'Hook 配置' },
  { view: 'history', icon: '◷', label: '历史记录' },
]

export function Sidebar({ currentView, onViewChange }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>Claude Monitor</h1>
      </div>
      <nav className="sidebar-nav">
        {navItems.map((item) => (
          <div
            key={item.view}
            className={`nav-item ${currentView === item.view ? 'active' : ''}`}
            onClick={() => onViewChange(item.view)}
          >
            <span className="nav-icon">{item.icon}</span>
            <span>{item.label}</span>
          </div>
        ))}
      </nav>
      <div className="sidebar-footer">
        <div>v1.0.0</div>
      </div>
    </aside>
  )
}
