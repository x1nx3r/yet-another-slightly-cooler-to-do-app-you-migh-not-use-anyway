mod input;
mod media;
mod tasks;
use tauri::Manager;

#[tauri::command]
fn exit_app() {
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

#[tauri::command]
fn get_accent_color() -> String {
    // Standard GNOME/GTK4 accent color mapping
    let default_accent = "#c084fc"; // Our default purple
    
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "accent-color"])
            .output();

        if let Ok(out) = output {
            let raw = String::from_utf8_lossy(&out.stdout).trim().to_string();
            // gsettings returns values like "'blue'" (with single quotes)
            let color_name = raw.trim_matches('\''); 

            match color_name {
                "blue" => return "#3584e4".to_string(),
                "teal" => return "#2190a4".to_string(),
                "green" => return "#3a944a".to_string(),
                "yellow" => return "#e5a50a".to_string(),
                "orange" => return "#ed5b00".to_string(),
                "red" => return "#e01b24".to_string(),
                "pink" => return "#d56199".to_string(),
                "purple" => return "#9141ac".to_string(),
                "slate" => return "#6f7e96".to_string(),
                _ => {}
            }
        }
    }

    default_accent.to_string()
}

pub fn run() {
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
            get_accent_color,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
