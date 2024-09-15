use serde_json::json;
use shared_child::SharedChild;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, Manager, State};

pub mod utils;

#[derive(Default)]
struct AppState {
    ffmpeg_processes: HashMap<String, Arc<SharedChild>>,
}

#[tauri::command]
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .creation_flags(utils::CREATE_NO_WINDOW)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
async fn compress(
    input_path: String,
    output_path: String,
    app: AppHandle,
    app_state: State<'_, Mutex<AppState>>,
) -> Result<bool, ()> {
    let state = app_state.lock().unwrap();
    if state.ffmpeg_processes.contains_key(&output_path) {
        return Ok(false);
    }
    drop(state);

    let duration = utils::get_video_duration(&input_path);

    let mut command = Command::new("ffmpeg");
    command
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
        .creation_flags(utils::CREATE_NO_WINDOW)
        .stdout(Stdio::piped());

    let shared_child = SharedChild::spawn(&mut command).unwrap();
    let child_arc = Arc::new(shared_child);

    let stdout = child_arc.take_stdout().unwrap();
    let reader = BufReader::new(stdout);

    thread::spawn(move || {
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
    });

    let mut state = app_state.lock().unwrap();
    state
        .ffmpeg_processes
        .insert(output_path.clone(), child_arc.clone());
    drop(state);

    let exit_status = child_arc.wait().unwrap();
    let mut state = app_state.lock().unwrap();
    state.ffmpeg_processes.remove(&output_path);

    Ok(exit_status.success())
}

#[tauri::command]
async fn cancel_compress(
    output_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, ()> {
    let mut state = state.lock().unwrap();
    if let Some(child) = state.ffmpeg_processes.remove(&output_path) {
        child.kill().expect("failed to kill child");
        thread::sleep(std::time::Duration::from_secs(1));
        if fs::metadata(&output_path).is_ok() {
            fs::remove_file(&output_path).unwrap();
        }
    }
    Ok(true)
}

#[tauri::command]
fn get_file_size(path: String) -> Result<u64, ()> {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .map_err(|_| ())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            is_ffmpeg_available,
            compress,
            cancel_compress,
            get_file_size
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
