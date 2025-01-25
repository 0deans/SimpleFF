use crate::state::AppState;
use std::{fs, sync::Mutex, thread};
use tauri::{AppHandle, State};

pub mod ffmpeg;

#[tauri::command]
pub fn close_request(app: AppHandle, state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();
    for (output_path, child) in state.ffmpeg_processes.iter() {
        child.kill().expect("failed to kill child");
        thread::sleep(std::time::Duration::from_secs(1));
        if fs::metadata(output_path).is_ok() {
            fs::remove_file(output_path).unwrap();
        }
    }
    app.exit(0);
}
