import { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { AppConfig } from '../types'

const defaultConfig: AppConfig = {
  hook_enabled: false,
  auto_start_monitor: true,
  polling_interval_secs: 5
}

export function useConfig() {
  const [config, setConfig] = useState<AppConfig>(defaultConfig)
  const [loading, setLoading] = useState(true)

  const loadConfig = useCallback(async () => {
    try {
      const loaded = await invoke<AppConfig>('get_config')
      const hookExists = await invoke<boolean>('is_hook_installed')
      setConfig({
        ...loaded,
        hook_enabled: loaded.hook_enabled && hookExists
      })
    } catch (e) {
      console.error('Failed to load config:', e)
    } finally {
      setLoading(false)
    }
  }, [])

  const saveConfig = useCallback(async (newConfig: AppConfig) => {
    try {
      await invoke('save_config', { config: newConfig })
      setConfig(newConfig)
    } catch (e) {
      console.error('Failed to save config:', e)
    }
  }, [])

  useEffect(() => {
    loadConfig()
  }, [loadConfig])

  return { config, loading, saveConfig, reload: loadConfig }
}
