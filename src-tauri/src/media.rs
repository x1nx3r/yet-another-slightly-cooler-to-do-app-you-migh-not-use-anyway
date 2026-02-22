use mpris::PlayerFinder;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
pub struct TrackInfo {
    title: String,
    artist: String,
    is_playing: bool,
}

pub fn init(app: &AppHandle) {
    let app_handle = app.clone();

    thread::spawn(move || {
        let player_finder = PlayerFinder::new().expect("Could not connect to D-Bus");

        // Simple polling for now, as event listener setup is more complex in mpris crate
        // Optimally we'd use a signal listener but for MVP polling is safer to implement quickly
        let mut last_title = String::new();
        let mut last_is_playing = false;

        loop {
            let mut sleep_duration = Duration::from_secs(1);

            match player_finder.find_active() {
                Ok(player) => {
                    let is_playing =
                        player.get_playback_status().ok() == Some(mpris::PlaybackStatus::Playing);

                    if !is_playing {
                        // Slow down polling when paused
                        sleep_duration = Duration::from_secs(3);
                    }

                    if let Ok(metadata) = player.get_metadata() {
                        let title = metadata.title().unwrap_or("Unknown").to_string();
                        let artist = metadata
                            .artists()
                            .map(|a| a.join(", "))
                            .unwrap_or_else(|| "".to_string());

                        if title != last_title || is_playing != last_is_playing {
                            let _ = app_handle.emit(
                                "media-change",
                                Some(TrackInfo {
                                    title: title.clone(),
                                    artist,
                                    is_playing,
                                }),
                            );
                            last_title = title;
                            last_is_playing = is_playing;
                        }
                    }
                }
                Err(_) => {
                    // No active player found - poll very slowly
                    sleep_duration = Duration::from_secs(10);

                    if !last_title.is_empty() {
                        let _ = app_handle.emit("media-change", None::<TrackInfo>);
                        last_title = String::new();
                        last_is_playing = false;
                    }
                }
            }
            thread::sleep(sleep_duration);
        }
    });
}

#[tauri::command]
pub fn get_current_track() -> Option<TrackInfo> {
    let player_finder = PlayerFinder::new().ok()?;
    let player = player_finder.find_active().ok()?;
    let is_playing = player.get_playback_status().ok() == Some(mpris::PlaybackStatus::Playing);
    let metadata = player.get_metadata().ok()?;

    let title = metadata.title().unwrap_or("Unknown").to_string();
    let artist = metadata
        .artists()
        .map(|a| a.join(", "))
        .unwrap_or_else(|| "".to_string());

    Some(TrackInfo {
        title,
        artist,
        is_playing,
    })
}
