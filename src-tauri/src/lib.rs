mod commands;
mod config;
mod database;
mod hook_server;
mod installer;
mod monitor;
mod models;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: Arc<Mutex<database::Database>>,
    pub monitor: Arc<Mutex<monitor::ProcessMonitor>>,
    pub hook_server: Arc<Mutex<hook_server::HookServer>>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(Mutex::new(database::Database::new().expect("Failed to init DB"))),
            monitor: Arc::new(Mutex::new(monitor::ProcessMonitor::new())),
            hook_server: Arc::new(Mutex::new(hook_server::HookServer::new(9876))),
        })
        .setup(|app| {
            let state = app.state::<AppState>();

            // 启动时初始化数据库
            tauri::async_runtime::block_on(async {
                let db = state.db.lock().await;
                db.init().expect("Failed to init database");
            });

            // 启动 hook 服务
            let hook_server = state.hook_server.clone();
            tauri::async_runtime::spawn(async move {
                let server = hook_server.lock().await;
                server.start().await;
            });

            // 启动监控循环
            let monitor = state.monitor.clone();
            let db = state.db.clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
                loop {
                    interval.tick().await;
                    let mut mon = monitor.lock().await;
                    let instances = mon.scan_instances().await;

                    let database = db.lock().await;
                    for instance in instances {
                        let _ = database.upsert_instance(&instance);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::check_claude_installed,
            commands::install_node,
            commands::install_claude,
            commands::get_instances,
            commands::get_instance_details,
            commands::get_instance_resources,
            commands::get_instance_sessions,
            commands::kill_instance,
            commands::get_hook_script,
            commands::install_hook,
            commands::uninstall_hook,
            commands::get_workspace_stats,
            commands::search_history,
            commands::get_config,
            commands::save_config,
            commands::is_hook_installed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
