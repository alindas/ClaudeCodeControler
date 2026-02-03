export interface InstallationStatus {
  installed: boolean
  version: string | null
  path: string | null
}

export interface ClaudeInstance {
  id: string
  pid: number
  cwd: string
  cmdline: string
  status: string
  start_time: string
  last_seen: string
  cpu_percent: number
  memory_mb: number
}

export interface InstanceResource {
  instance_id: string
  timestamp: string
  cpu_percent: number
  memory_mb: number
}

export interface SessionEvent {
  id: string
  instance_id: string
  event_type: string
  content: string
  timestamp: string
  metadata: string | null
}

export interface WorkspaceStats {
  path: string
  name: string
  session_count: number
  total_tokens: number
  last_active: string | null
}

export interface AppConfig {
  hook_enabled: boolean
  auto_start_monitor: boolean
  polling_interval_secs: number
}

export type ViewType = 'dashboard' | 'install' | 'hooks' | 'history'
