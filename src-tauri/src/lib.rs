use serde_json::json;
use std::{
    io::{BufRead, BufReader}, os::windows::process::CommandExt, process::{Command, Stdio}
};
use tauri::{AppHandle, Emitter};

pub mod utils;

#[tauri::command]
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
async fn compress(input_path: String, output_path: String, app: AppHandle) -> bool {
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

    let status = child.wait().expect("Failed to wait on child process");
    status.success()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![is_ffmpeg_available, compress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
