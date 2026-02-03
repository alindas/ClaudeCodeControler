import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { AppConfig } from '../types'

export function useHook(config: AppConfig, saveConfig: (c: AppConfig) => Promise<void>) {
  const [installing, setInstalling] = useState(false)

  const installHook = useCallback(async () => {
    setInstalling(true)
    try {
      const result = await invoke<string>('install_hook')
      const newConfig = { ...config, hook_enabled: true }
      await saveConfig(newConfig)
      return result
    } catch (e) {
      throw e
    } finally {
      setInstalling(false)
    }
  }, [config, saveConfig])

  const uninstallHook = useCallback(async () => {
    setInstalling(true)
    try {
      const result = await invoke<string>('uninstall_hook')
      const newConfig = { ...config, hook_enabled: false }
      await saveConfig(newConfig)
      return result
    } catch (e) {
      throw e
    } finally {
      setInstalling(false)
    }
  }, [config, saveConfig])

  const toggleHook = useCallback(async () => {
    if (config.hook_enabled) {
      await uninstallHook()
    } else {
      await installHook()
    }
  }, [config.hook_enabled, installHook, uninstallHook])

  return {
    installing,
    installHook,
    uninstallHook,
    toggleHook
  }
}
