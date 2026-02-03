import React, { useState, useEffect, useCallback } from 'react'
import ReactDOM from 'react-dom/client'
import { invoke } from '@tauri-apps/api/tauri'
import './style.css'

import { Sidebar, Dashboard, Install, Hooks, History } from './components'
import { useConfig, useInstances, useHook, useInstaller } from './hooks'
import { ViewType } from './types'

function App() {
  const [currentView, setCurrentView] = useState<ViewType>('dashboard')
  const { config, loading: configLoading, saveConfig } = useConfig()
  const { instances, installStatus, loading, refresh, killInstance, startPolling } = useInstances()
  const { installing: hookInstalling, toggleHook } = useHook(config, saveConfig)
  const { installingNode, installingClaude, installNode, installClaude } = useInstaller()

  // 启动轮询
  useEffect(() => {
    if (currentView === 'dashboard') {
      const stop = startPolling(5000)
      return stop
    }
  }, [currentView, startPolling])

  const handleToggleHook = useCallback(async () => {
    await toggleHook()
  }, [toggleHook])

  const renderContent = () => {
    switch (currentView) {
      case 'dashboard':
        return (
          <Dashboard
            instances={instances}
            installStatus={installStatus}
            loading={loading}
            onRefresh={refresh}
            onKillInstance={killInstance}
          />
        )
      case 'install':
        return (
          <Install
            installStatus={installStatus}
            installingNode={installingNode}
            installingClaude={installingClaude}
            onInstallNode={installNode}
            onInstallClaude={installClaude}
            onRefresh={refresh}
          />
        )
      case 'hooks':
        return (
          <Hooks
            config={config}
            installing={hookInstalling}
            onToggle={handleToggleHook}
          />
        )
      case 'history':
        return <History />
      default:
        return null
    }
  }

  return (
    <div className="container">
      <Sidebar currentView={currentView} onViewChange={setCurrentView} />
      <main className="main">
        {renderContent()}
      </main>
    </div>
  )
}

const root = document.getElementById('root')
if (root) {
  ReactDOM.createRoot(root).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  )
}
