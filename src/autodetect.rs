use crate::*;
use std::path::{Path, PathBuf};

/// A choice between detecting 32-bit and 64-bit program folders and executables.
#[derive(Clone, Copy, Debug)]
pub enum AutodetectType {
    Default64BitMachine,
    Default32BitMachine,
}

impl AutodetectType {
    pub fn exe_name(&self) -> String {
        match self {
            Self::Default32BitMachine => "wallpaper32.exe".to_string(),
            Self::Default64BitMachine => "wallpaper64.exe".to_string(),
        }
    }
}

/// Tries to autodetect the Wallpaper Engine workshop content folder.
pub fn autodetect_wallpaper_engine_workshop(loc: AutodetectType) -> Option<PathBuf> {
    match loc {
        AutodetectType::Default64BitMachine => {
            let path =
                Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\431960");
            if std::fs::exists(path).unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        }
        AutodetectType::Default32BitMachine => {
            // " (x86)" missing on 32 bit machines since it applies to all programs
            let path = Path::new("C:\\Program Files\\Steam\\steamapps\\workshop\\content\\431960");
            if std::fs::exists(path).unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        }
    }
}

/// Tries to autodetect the installed Wallpaper Engine workshop wallpapers.
pub fn autodetect_wallpaper_engine_wallpapers(loc: AutodetectType) -> Option<Vec<Wallpaper>> {
    if let Some(content_folder) = autodetect_wallpaper_engine_workshop(loc) {
        let mut vec = vec![];
        let globbed =
            glob::glob(&(content_folder.to_string_lossy().to_string() + "/*/project.json"))
                .unwrap()
                .flatten()
                .collect::<Vec<PathBuf>>();
        for path in globbed {
            if let Ok(content) = std::fs::read_to_string(path.clone()) {
                if let Ok(project) = serde_json::from_str::<ProjectJson>(&content) {
                    vec.push(Wallpaper {
                        name: project.title,
                        _type: project._type,
                        path,
                    });
                }
            }
        }
        return Some(vec);
    }
    None
}

/// Tries to autodetect the Wallpaper Engine workshop config file.
pub fn autodetect_wallpaper_engine_config(loc: AutodetectType) -> Option<PathBuf> {
    match loc {
        AutodetectType::Default64BitMachine => {
            let path = Path::new(
                "C:\\Program Files (x86)\\Steam\\steamapps\\common\\wallpaper_engine\\config.json",
            );
            if std::fs::exists(path).unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        }
        AutodetectType::Default32BitMachine => {
            let path = Path::new(
                "C:\\Program Files\\Steam\\steamapps\\common\\wallpaper_engine\\config.json",
            );
            if std::fs::exists(path).unwrap_or(false) {
                Some(path.to_path_buf())
            } else {
                None
            }
        }
    }
}

/// Tries to autodetect the Wallpaper Engine main folder.
pub fn autodetect_wallpaper_engine_folder_from_config(config: PathBuf) -> Option<PathBuf> {
    if let Ok(content) = std::fs::read_to_string(config) {
        let config = serde_json::from_str::<WallpaperEngineConfigJson>(&content);
        if let Ok(result) = config {
            let path = PathBuf::from(&result.install_directory);
            if let Ok(exists) = std::fs::exists(path.as_path()) {
                if exists {
                    return Some(path);
                }
            }
        }
    }
    None
}

/// Tries to autodetect the Wallpaper Engine executable.
pub fn autodetect_wallpaper_engine_exe_from_config(
    config: PathBuf,
    loc: AutodetectType,
) -> Option<PathBuf> {
    if let Some(folder) = autodetect_wallpaper_engine_folder_from_config(config) {
        let path = PathBuf::from(&(folder.to_string_lossy().to_string() + "/" + &loc.exe_name()));
        if let Ok(exists) = std::fs::exists(path.as_path()) {
            if exists {
                return Some(path);
            }
        }
    }
    None
}
