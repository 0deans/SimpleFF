use crate::{models::VideoParams, state::AppState, utils};
use serde_json::json;
use shared_child::SharedChild;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
    fs,
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub fn is_ffmpeg_available() -> Result<bool, String> {
    let mut command = Command::new("ffmpeg");
    command.arg("-version");

    #[cfg(target_os = "windows")]
    command.creation_flags(crate::utils::CREATE_NO_WINDOW);

    command
        .output()
        .map(|output| output.status.success())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_video(
    params: VideoParams,
    app: AppHandle,
    app_state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    {
        let state = app_state.lock().unwrap();
        if state.ffmpeg_processes.contains_key(&params.output_path) {
            return Err(format!(
                "Another compression process is already running for {}",
                params.output_path
            ));
        }
    }

    let duration = utils::ffmpeg::get_video_duration(&params.input_path);

    let mut command = Command::new("ffmpeg");
    command.arg("-y").arg("-i").arg(&params.input_path);

    if let Some(video_code) = params.video_codec {
        command.arg("-c:v").arg(video_code);
    }

    if let Some(audio_codec) = params.audio_codec {
        command.arg("-c:a").arg(audio_codec);
    }

    if let Some(params) = params.codec_params {
        for (key, value) in params {
            command.arg(format!("-{}", key)).arg(value);
        }
    }

    command
        .arg("-progress")
        .arg("pipe:1")
        .arg("-loglevel")
        .arg("error")
        .arg(&params.output_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(target_os = "windows")]
    command.creation_flags(utils::CREATE_NO_WINDOW);

    let shared_child = SharedChild::spawn(&mut command).unwrap();
    let child_arc = Arc::new(shared_child);

    let stdout = child_arc.take_stdout().unwrap();
    let stderr = child_arc.take_stderr().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("out_time=") {
                    if let Some(out_time) = line.split('=').nth(1) {
                        if let Some(out_time) = utils::ffmpeg::parse_time(out_time) {
                            let payload = json!({
                                "filePath": params.input_path,
                                "percentage": (out_time / duration) * 100.0
                            });
                            app.emit("compress:progress", payload.to_string()).unwrap();
                        }
                    }
                }
            }
        }
    });

    let error = Arc::new(Mutex::new(String::new()));
    let error_clone = error.clone();

    thread::spawn(move || {
        let mut error = error_clone.lock().unwrap();
        let mut reader = BufReader::new(stderr);
        reader.read_to_string(&mut *error).unwrap();
    });

    {
        let mut state = app_state.lock().unwrap();
        state
            .ffmpeg_processes
            .insert(params.output_path.clone(), child_arc.clone());
    }

    let exit_status = child_arc.wait().unwrap();
    let mut state = app_state.lock().unwrap();
    state.ffmpeg_processes.remove(&params.output_path);

    let error_message = error.lock().unwrap().clone();
    if !error_message.is_empty() {
        return Err(error_message);
    }

    if !exit_status.success() {
        return Err("FFmpeg process failed with non-zero exit status".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn cancel_video_processing(
    output_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let child = {
        let mut state = state
            .lock()
            .map_err(|e| format!("State lock error: {}", e))?;
        state
            .ffmpeg_processes
            .remove(&output_path)
            .ok_or_else(|| format!("Process not found for {}", output_path))?
    };

    if let Err(e) = child.kill() {
        return Err(format!("Failed to kill child: {}", e));
    }

    let _ = child.wait().map_err(|e| format!("Wait failed: {}", e))?;

    match fs::remove_file(&output_path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("Failed to remove file: {}", e)),
    }

    Ok(())
}
