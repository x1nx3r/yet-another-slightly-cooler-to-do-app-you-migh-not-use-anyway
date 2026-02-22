mod input;
mod media;
mod tasks;
use tauri::Manager;

/// Reclaim the reserved monitor space
fn cleanup_reserved_space() {
    let _ = std::process::Command::new("hyprctl")
        .args(["keyword", "monitor", ",addreserved,0,0,0,0"])
        .output();
}

#[tauri::command]
fn exit_app() {
    cleanup_reserved_space();
    std::process::exit(0);
}

#[tauri::command]
fn toggle_settings_window(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("settings") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Handle Unix signals: reclaim reserved space before exiting
    let mut signals = signal_hook::iterator::Signals::new(&[
        signal_hook::consts::signal::SIGINT,
        signal_hook::consts::signal::SIGTERM,
        signal_hook::consts::signal::SIGHUP,
    ])
    .expect("Error setting signal handler");

    std::thread::spawn(move || {
        for _ in signals.forever() {
            cleanup_reserved_space();
            std::process::exit(0);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize Task State
            let task_state = tasks::init(app.handle());
            app.manage(task_state);

            // Initialize Input Monitoring (Background Thread)
            input::init(app.handle());

            // Initialize Media Monitoring (Background Thread)
            media::init(app.handle());

            // Reserve 36px at the bottom of the monitor for the dock
            let _ = std::process::Command::new("hyprctl")
                .args(["keyword", "monitor", ",addreserved,0,30,0,0"])
                .output();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tasks::get_tasks,
            tasks::add_task,
            tasks::toggle_task,
            tasks::delete_task,
            tasks::set_active_task,
            tasks::get_active_task,
            media::get_current_track,
            toggle_settings_window,
            exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Normal exit — also reclaim reserved space
    cleanup_reserved_space();
}
