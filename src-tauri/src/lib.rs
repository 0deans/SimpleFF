use serde_json::json;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
};
use tauri::{AppHandle, Emitter, Manager, State};

pub mod utils;

#[derive(Default)]
struct AppState {
    ffmpeg_processes: HashMap<String, Child>,
}

#[tauri::command]
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
async fn compress(
    input_path: String,
    output_path: String,
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, ()> {
    let mut state_lock = state.lock().unwrap();
    let duration = utils::get_video_duration(&input_path);

    let mut child = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            &input_path,
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            "-crf",
            "26",
            "-progress",
            "pipe:1",
            &output_path,
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start process");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    state_lock
        .ffmpeg_processes
        .insert(output_path.clone(), child);
    drop(state_lock);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("out_time=") {
                if let Some(out_time) = line.split('=').nth(1) {
                    if let Some(out_time) = utils::parse_time(out_time) {
                        let payload = json!({
                            "filePath": input_path,
                            "percentage": (out_time / duration) * 100.0
                        });
                        app.emit("compress:progress", payload.to_string()).unwrap();
                    }
                }
            }
        }
    }

    Ok(true)
}

#[tauri::command]
async fn cancel_compress(
    output_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, ()> {
    let mut state_lock = state.lock().unwrap();
    if let Some(mut child) = state_lock.ffmpeg_processes.remove(&output_path) {
        child.kill().expect("Failed to kill ffmpeg process");
        thread::sleep(std::time::Duration::from_secs(1));
        if fs::metadata(&output_path).is_ok() {
            fs::remove_file(&output_path).expect("Failed to delete file");
        }
    }
    Ok(true)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            is_ffmpeg_available,
            compress,
            cancel_compress
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
