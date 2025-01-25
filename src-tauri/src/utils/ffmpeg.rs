#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn get_video_duration(filepath: &String) -> f64 {
    let mut command = Command::new("ffprobe");
    command.args(&[
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        filepath,
    ]);

    #[cfg(target_os = "windows")]
    command.creation_flags(crate::utils::CREATE_NO_WINDOW);

    let output = command.output().expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let duration: f64 = output_str.trim().parse().unwrap();
    duration
}

pub fn parse_time(time: &str) -> Option<f64> {
    let mut time_parts = time.split(':').map(|s| s.parse::<f64>());

    let hours = time_parts.next()?.ok()?;
    let minutes = time_parts.next()?.ok()?;
    let seconds = time_parts.next()?.ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}
