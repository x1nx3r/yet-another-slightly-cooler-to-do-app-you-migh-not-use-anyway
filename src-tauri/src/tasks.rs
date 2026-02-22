use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::thread;
use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub completed: bool,
    #[serde(default)]
    pub active: bool,
    /// Unix timestamp (seconds) when this task was set as active
    #[serde(default)]
    pub started_at: Option<i64>,
}

#[derive(Default)]
pub struct TaskState {
    pub tasks: Mutex<Vec<Task>>,
    pub file_path: PathBuf,
}

/// Save tasks to disk and emit a global event in a non-blocking background thread.
fn save_and_notify(tasks: Vec<Task>, path: PathBuf, app: AppHandle) {
    thread::spawn(move || {
        if let Ok(json) = serde_json::to_string(&tasks) {
            let _ = fs::write(path, json);
        }
        // Broadcast to ALL windows (dock + settings)
        let _ = app.emit("tasks-updated", ());
    });
}

#[tauri::command]
pub fn get_tasks(state: State<'_, TaskState>) -> Vec<Task> {
    state.tasks.lock().clone()
}

#[tauri::command]
pub fn add_task(app: AppHandle, text: String, state: State<'_, TaskState>) -> Vec<Task> {
    let mut tasks = state.tasks.lock();
    let new_task = Task {
        id: Uuid::new_v4().to_string(),
        text,
        completed: false,
        active: false,
        started_at: None,
    };
    tasks.push(new_task);
    save_and_notify(tasks.clone(), state.file_path.clone(), app);
    tasks.clone()
}

#[tauri::command]
pub fn toggle_task(app: AppHandle, id: String, state: State<'_, TaskState>) -> Vec<Task> {
    let mut tasks = state.tasks.lock();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = !task.completed;
        // If completing an active task, deactivate it
        if task.completed && task.active {
            task.active = false;
            task.started_at = None;
        }
    }
    save_and_notify(tasks.clone(), state.file_path.clone(), app);
    tasks.clone()
}

#[tauri::command]
pub fn delete_task(app: AppHandle, id: String, state: State<'_, TaskState>) -> Vec<Task> {
    let mut tasks = state.tasks.lock();
    tasks.retain(|t| t.id != id);
    save_and_notify(tasks.clone(), state.file_path.clone(), app);
    tasks.clone()
}

#[tauri::command]
pub fn set_active_task(app: AppHandle, id: String, state: State<'_, TaskState>) -> Vec<Task> {
    let mut tasks = state.tasks.lock();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for task in tasks.iter_mut() {
        if task.id == id {
            // Toggle: if already active, deactivate
            if task.active {
                task.active = false;
                task.started_at = None;
            } else {
                task.active = true;
                task.started_at = Some(now);
            }
        } else {
            task.active = false;
            task.started_at = None;
        }
    }
    save_and_notify(tasks.clone(), state.file_path.clone(), app);
    tasks.clone()
}

#[tauri::command]
pub fn get_active_task(state: State<'_, TaskState>) -> Option<Task> {
    state.tasks.lock().iter().find(|t| t.active).cloned()
}

pub fn init(app: &AppHandle) -> TaskState {
    let app_dir = app.path().app_data_dir().unwrap_or(PathBuf::from("."));
    if !app_dir.exists() {
        let _ = fs::create_dir_all(&app_dir);
    }
    let file_path = app_dir.join("tasks.json");

    let tasks = if file_path.exists() {
        if let Ok(content) = fs::read_to_string(&file_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    TaskState {
        tasks: Mutex::new(tasks),
        file_path,
    }
}
