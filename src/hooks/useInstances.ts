import { useState, useEffect, useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { ClaudeInstance, InstallationStatus } from '../types'

export function useInstances() {
  const [instances, setInstances] = useState<ClaudeInstance[]>([])
  const [installStatus, setInstallStatus] = useState<InstallationStatus | null>(null)
  const [loading, setLoading] = useState(true)
  const intervalRef = useRef<NodeJS.Timeout | null>(null)

  const checkInstallation = useCallback(async () => {
    try {
      const status = await invoke<InstallationStatus>('check_claude_installed')
      setInstallStatus(status)
    } catch (e) {
      console.error('Failed to check installation:', e)
    }
  }, [])

  const loadInstances = useCallback(async () => {
    try {
      const data = await invoke<ClaudeInstance[]>('get_instances', { activeOnly: true })
      setInstances(data)
    } catch (e) {
      console.error('Failed to load instances:', e)
    } finally {
      setLoading(false)
    }
  }, [])

  const killInstance = useCallback(async (pid: number) => {
    try {
      await invoke('kill_instance', { pid })
      await loadInstances()
    } catch (e) {
      alert('终止失败: ' + e)
    }
  }, [loadInstances])

  const refresh = useCallback(async () => {
    setLoading(true)
    await Promise.all([checkInstallation(), loadInstances()])
    setLoading(false)
  }, [checkInstallation, loadInstances])

  const startPolling = useCallback((intervalMs = 5000) => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current)
    }
    intervalRef.current = setInterval(() => {
      loadInstances()
    }, intervalMs)
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [loadInstances])

  useEffect(() => {
    refresh()
  }, [refresh])

  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current)
      }
    }
  }, [])

  return {
    instances,
    installStatus,
    loading,
    refresh,
    killInstance,
    startPolling,
    stopPolling: () => intervalRef.current && clearInterval(intervalRef.current)
  }
}
