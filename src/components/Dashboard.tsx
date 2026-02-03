import { useState, useMemo } from 'react'
import { ClaudeInstance, InstallationStatus } from '../types'

interface DashboardProps {
  instances: ClaudeInstance[]
  installStatus: InstallationStatus | null
  loading: boolean
  onRefresh: () => void
  onKillInstance: (pid: number) => void
}

interface InstanceGroup {
  cwd: string
  instances: ClaudeInstance[]
  totalCpu: number
  totalMem: number
}

function formatTime(isoString: string): string {
  const date = new Date(isoString)
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

export function Dashboard({
  instances,
  installStatus,
  loading,
  onRefresh,
  onKillInstance
}: DashboardProps) {
  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set())

  const groups = useMemo(() => {
    const grouped = instances.reduce((acc, inst) => {
      const key = inst.cwd || '未知路径'
      if (!acc[key]) acc[key] = []
      acc[key].push(inst)
      return acc
    }, {} as Record<string, ClaudeInstance[]>)

    return Object.entries(grouped)
      .map(([cwd, insts]): InstanceGroup => ({
        cwd,
        instances: insts,
        totalCpu: insts.reduce((sum, i) => sum + i.cpu_percent, 0),
        totalMem: insts.reduce((sum, i) => sum + i.memory_mb, 0)
      }))
      .sort((a, b) => a.cwd.localeCompare(b.cwd))
  }, [instances])

  const toggleGroup = (cwd: string) => {
    setExpandedGroups(prev => {
      const next = new Set(prev)
      if (next.has(cwd)) {
        next.delete(cwd)
      } else {
        next.add(cwd)
      }
      return next
    })
  }

  if (!installStatus?.installed) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">!</div>
        <h3>Claude Code 未安装</h3>
        <p>请先前往安装管理页面完成安装</p>
      </div>
    )
  }

  return (
    <>
      <div className="main-header">
        <h2>运行中的实例 ({instances.length})</h2>
        <button className="btn btn-primary" onClick={onRefresh} disabled={loading}>
          {loading ? '刷新中...' : '刷新'}
        </button>
      </div>

      <div className="content-scroll">
      {instances.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">○</div>
          <h3>暂无运行中的实例</h3>
          <p>使用命令行启动 Claude Code 后将自动显示在这里</p>
        </div>
      ) : (
        <div className="instance-list">
          {groups.map((group) => {
            const isExpanded = expandedGroups.has(group.cwd)
            return (
              <div key={group.cwd} className="instance-group">
                <div
                  className="group-header"
                  onClick={() => toggleGroup(group.cwd)}
                >
                  <div className="group-toggle">
                    <span className={`group-icon ${isExpanded ? 'expanded' : ''}`}>▶</span>
                  </div>
                  <div className="group-info">
                    <div className="group-path" title={group.cwd}>{group.cwd}</div>
                    <div className="group-meta">
                      <span className="group-count">{group.instances.length} 个进程</span>
                      <span>CPU: {group.totalCpu.toFixed(1)}%</span>
                      <span>MEM: {group.totalMem.toFixed(1)} MB</span>
                    </div>
                  </div>
                </div>

                <div className={`group-instances ${isExpanded ? 'expanded' : ''}`}>
                  {group.instances.map((inst) => (
                    <div key={inst.id} className="instance-item">
                      <div className="instance-info">
                        <div className="instance-pid">PID: {inst.pid}</div>
                        <div
                          className="instance-cmdline"
                          title={inst.cmdline}
                        >
                          {inst.cmdline.length > 60
                            ? inst.cmdline.substring(0, 60) + '...'
                            : inst.cmdline}
                        </div>
                      </div>
                      <div className="instance-meta">
                        <span>CPU: {inst.cpu_percent.toFixed(1)}%</span>
                        <span>MEM: {inst.memory_mb.toFixed(1)} MB</span>
                        <span>启动: {formatTime(inst.start_time)}</span>
                      </div>
                      <span className="badge badge-running">
                        <span className="badge-dot"></span>
                        运行中
                      </span>
                      <div className="instance-actions">
                        <button
                          className="btn btn-sm btn-danger"
                          onClick={(e) => {
                            e.stopPropagation()
                            if (confirm(`确定要终止 PID ${inst.pid} 的实例吗？`)) {
                              onKillInstance(inst.pid)
                            }
                          }}
                        >
                          终止
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )
          })}
        </div>
      )}
      </div>
    </>
  )
}
