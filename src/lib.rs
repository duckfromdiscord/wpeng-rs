pub mod autodetect;
pub mod json;

use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, Output};

use crate::json::{ProjectJson, WallpaperEngineConfigJson};

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub name: String,
    pub _type: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub name: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub x: Option<u16>,
    pub y: Option<u16>,
    pub activate: bool,
    pub borderless: bool,
}

/// Pauses all wallpapers.
pub fn pause_all(exe_path: PathBuf) -> Result<Output, Error> {
    Command::new(exe_path.as_path().as_os_str())
        .arg("-control")
        .arg("pause")
        .output()
}

/// Resumes all wallpapers.
pub fn resume_all(exe_path: PathBuf) -> Result<Output, Error> {
    Command::new(exe_path.as_path().as_os_str())
        .arg("-control")
        .arg("play")
        .output()
}

/// Stops all wallpapers.
pub fn stop_all(exe_path: PathBuf) -> Result<Output, Error> {
    Command::new(exe_path.as_path().as_os_str())
        .arg("-control")
        .arg("stop")
        .output()
}

/// Mutes all wallpapers.
pub fn mute_all(exe_path: PathBuf) -> Result<Output, Error> {
    Command::new(exe_path.as_path().as_os_str())
        .arg("-control")
        .arg("mute")
        .output()
}

/// Unmutes all wallpapers.
pub fn unmute_all(exe_path: PathBuf) -> Result<Output, Error> {
    Command::new(exe_path.as_path().as_os_str())
        .arg("-control")
        .arg("unmute")
        .output()
}

/// Opens a wallpaper, given a `project.json`. Allows you to select a monitor by index, or play in a window.
pub fn open_wallpaper(
    exe_path: PathBuf,
    file: String,
    location: Option<String>,
    monitor: Option<u8>,
    preset: Option<String>,
    play_in_window: Option<WindowSettings>,
) -> Result<Output, Error> {
    let mut cmd: &mut Command = &mut Command::new(exe_path.as_path().as_os_str());
    cmd = cmd
        .arg("-control")
        .arg("openWallpaper")
        .arg("-file")
        .arg(file);
    if let Some(location) = location {
        cmd = cmd.arg("-location").arg(location);
    }
    if let Some(monitor) = monitor {
        cmd = cmd.arg("-monitor").arg(monitor.to_string());
    }
    if let Some(preset) = preset {
        cmd = cmd.arg("-preset").arg(preset);
    }
    if let Some(play_in_window) = play_in_window {
        if let Some(window_name) = play_in_window.name {
            cmd = cmd.arg("-playInWindow").arg(window_name);
        } else {
            cmd = cmd.arg("-playInWindow");
        }
        if let Some(width) = play_in_window.width {
            cmd = cmd.arg("-width").arg(width.to_string());
        }
        if let Some(height) = play_in_window.height {
            cmd = cmd.arg("-height").arg(height.to_string());
        }
        if let Some(x) = play_in_window.x {
            cmd = cmd.arg("-x").arg(x.to_string());
        }
        if let Some(y) = play_in_window.y {
            cmd = cmd.arg("-y").arg(y.to_string());
        }
        if play_in_window.activate {
            cmd = cmd.arg("-activate");
        }
        if play_in_window.borderless {
            cmd = cmd.arg("-borderless");
        }
    }
    cmd.output()
}
