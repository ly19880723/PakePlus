mod command;
use std::sync::{Arc, Mutex};
mod utils;
use command::model::ServerState;
use tauri::menu::*;

pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(ServerState {
            server_handle: None,
        })))
        .menu(|handle| {
            let menu = Menu::with_items(
                handle,
                &[
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "Edit",
                        true,
                        &[
                            &PredefinedMenuItem::undo(handle, None)?,
                            &PredefinedMenuItem::redo(handle, None)?,
                            &PredefinedMenuItem::cut(handle, None)?,
                            &PredefinedMenuItem::copy(handle, None)?,
                            &PredefinedMenuItem::paste(handle, None)?,
                            &PredefinedMenuItem::select_all(handle, None)?,
                        ],
                    )?,
                ],
            );
            menu
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            command::cmds::preview_from_config,
            command::cmds::open_url,
            command::cmds::open_devtools,
            command::cmds::update_init_rs,
            command::cmds::start_server,
            command::cmds::stop_server,
            command::cmds::get_machine_uid,
            command::cmds::compress_folder,
            command::cmds::decompress_file,
            command::cmds::download_file,
            command::cmds::notification,
            command::cmds::run_command,
            command::cmds::get_env_var,
            command::cmds::find_port,
            command::cmds::get_exe_dir,
            command::cmds::windows_build,
            command::cmds::macos_build,
            command::cmds::linux_build,
            command::cmds::build_local,
            command::cmds::get_workflow_yml,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            // WebView 创建必须在主线程执行，不能 spawn 到异步任务
            if let Err(e) = utils::init::resolve_setup_sync(app_handle) {
                eprintln!("[PakePlus] 初始化失败: {}", e);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
