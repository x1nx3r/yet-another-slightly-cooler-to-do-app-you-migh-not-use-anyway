use evdev::{InputEventKind, Key};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn init(app: &AppHandle) {
    let app_handle = app.clone();

    thread::spawn(move || {
        loop {
            // Find ALL keyboard devices, not just the first one
            let devices: Vec<_> = evdev::enumerate()
                .map(|(_, d)| d)
                .filter(|d| {
                    d.supported_keys()
                        .is_some_and(|keys| keys.contains(Key::KEY_A))
                })
                .collect();

            if devices.is_empty() {
                eprintln!("Input: No keyboard found. Retrying in 5s...");
                thread::sleep(Duration::from_secs(5));
                continue;
            }

            // Spawn a listener thread for each keyboard device
            let mut handles = Vec::new();
            for device in devices {
                let name = device.name().unwrap_or("Unknown").to_string();
                println!("Input: Listening on keyboard: {}", name);

                let app_clone = app_handle.clone();
                let handle = thread::spawn(move || {
                    listen_device(device, &app_clone);
                });
                handles.push(handle);
            }

            // Wait for all listener threads (they exit on error/disconnect)
            for h in handles {
                let _ = h.join();
            }

            // If we get here, a device disconnected. Re-enumerate after a brief pause.
            eprintln!("Input: Device disconnected. Re-enumerating in 2s...");
            thread::sleep(Duration::from_secs(2));
        }
    });
}

fn listen_device(mut device: evdev::Device, app: &AppHandle) {
    while let Ok(events) = device.fetch_events() {
        for event in events {
            if let InputEventKind::Key(key) = event.kind() {
                if event.value() == 1 {
                    let key_name = format_key(key);
                    let _ = app.emit("keystroke-event", key_name);
                }
            }
        }
    }
}

fn format_key(key: Key) -> String {
    match key {
        Key::KEY_SPACE => "␣".to_string(),
        Key::KEY_ENTER => "↵".to_string(),
        Key::KEY_BACKSPACE => "⌫".to_string(),
        Key::KEY_TAB => "⇥".to_string(),
        Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => "⇧".to_string(),
        Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => "Ctrl".to_string(),
        Key::KEY_LEFTALT | Key::KEY_RIGHTALT => "Alt".to_string(),
        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => "Super".to_string(),
        Key::KEY_ESC => "Esc".to_string(),
        Key::KEY_CAPSLOCK => "Caps".to_string(),
        Key::KEY_UP => "↑".to_string(),
        Key::KEY_DOWN => "↓".to_string(),
        Key::KEY_LEFT => "←".to_string(),
        Key::KEY_RIGHT => "→".to_string(),
        // Fast path for letters and numbers to avoid Debug formatting
        Key::KEY_A => "a".to_string(),
        Key::KEY_B => "b".to_string(),
        Key::KEY_C => "c".to_string(),
        Key::KEY_D => "d".to_string(),
        Key::KEY_E => "e".to_string(),
        Key::KEY_F => "f".to_string(),
        Key::KEY_G => "g".to_string(),
        Key::KEY_H => "h".to_string(),
        Key::KEY_I => "i".to_string(),
        Key::KEY_J => "j".to_string(),
        Key::KEY_K => "k".to_string(),
        Key::KEY_L => "l".to_string(),
        Key::KEY_M => "m".to_string(),
        Key::KEY_N => "n".to_string(),
        Key::KEY_O => "o".to_string(),
        Key::KEY_P => "p".to_string(),
        Key::KEY_Q => "q".to_string(),
        Key::KEY_R => "r".to_string(),
        Key::KEY_S => "s".to_string(),
        Key::KEY_T => "t".to_string(),
        Key::KEY_U => "u".to_string(),
        Key::KEY_V => "v".to_string(),
        Key::KEY_W => "w".to_string(),
        Key::KEY_X => "x".to_string(),
        Key::KEY_Y => "y".to_string(),
        Key::KEY_Z => "z".to_string(),
        Key::KEY_0 => "0".to_string(),
        Key::KEY_1 => "1".to_string(),
        Key::KEY_2 => "2".to_string(),
        Key::KEY_3 => "3".to_string(),
        Key::KEY_4 => "4".to_string(),
        Key::KEY_5 => "5".to_string(),
        Key::KEY_6 => "6".to_string(),
        Key::KEY_7 => "7".to_string(),
        Key::KEY_8 => "8".to_string(),
        Key::KEY_9 => "9".to_string(),
        other => {
            let raw = format!("{:?}", other);
            raw.strip_prefix("KEY_").unwrap_or(&raw).to_lowercase()
        }
    }
}
