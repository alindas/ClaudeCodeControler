import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

export function useInstaller() {
  const [installingNode, setInstallingNode] = useState(false)
  const [installingClaude, setInstallingClaude] = useState(false)

  const installNode = useCallback(async () => {
    setInstallingNode(true)
    try {
      const result = await invoke<string>('install_node')
      return result
    } catch (e) {
      throw e
    } finally {
      setInstallingNode(false)
    }
  }, [])

  const installClaude = useCallback(async () => {
    setInstallingClaude(true)
    try {
      const result = await invoke<string>('install_claude')
      return result
    } catch (e) {
      throw e
    } finally {
      setInstallingClaude(false)
    }
  }, [])

  return {
    installingNode,
    installingClaude,
    installNode,
    installClaude
  }
}
