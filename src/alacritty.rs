use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum SupportedPlatform {
    Unix,
    Windows,
}

pub fn detect_platform() -> Result<SupportedPlatform, String> {
    return match env::consts::OS {
        "linux" | "macos" => Ok(SupportedPlatform::Unix),
        "windows" => Ok(SupportedPlatform::Windows),
        other => Err(format!("Unsupported platform: {}", other)),
    };
}

/// Gets the path to the Alacritty configuration file for the given platform
pub fn get_config_file_path(platform: SupportedPlatform) -> Result<PathBuf, String> {
    let mut possible_locations: Vec<String> = Vec::new();
    let config_file_name = "alacritty.toml";

    if let SupportedPlatform::Unix = platform {
        // $XDG_CONFIG_HOME/alacritty/alacritty.toml
        // $XDG_CONFIG_HOME/alacritty.toml
        // $HOME/.config/alacritty/alacritty.toml
        // $HOME/.alacritty.toml
        let _ = env::var("XDG_CONFIG_HOME").inspect(|val| {
            possible_locations.push(format!("{}/alacritty/{}", val, config_file_name));
            possible_locations.push(format!("{}/{}", val, config_file_name));
        });

        let _ = env::var("HOME").inspect(|val| {
            possible_locations.push(format!("{}/.config/alacritty/{}", val, config_file_name));
            possible_locations.push(format!("{}/{}", val, config_file_name));
        });
    } else if let SupportedPlatform::Windows = platform {
        // %APPDATA%\alacritty\alacritty.toml
        let _ = env::var("APPDATA").inspect(|val| {
            possible_locations.push(format!("{}\\alacritty\\{}", val, config_file_name));
        });
    }

    for config_file in possible_locations {
        let path = Path::new(&config_file);
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    Err(String::from("Could not find configuration file"))
}

// Gets all the theme available in a given themes directory
pub fn get_themes() -> Result<Vec<PathBuf>, String> {
    let mut themes_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    themes_dir.push("themes");

    if !themes_dir.exists() {
        return Err("Themes directory does not exist".to_string());
    }

    let themes = fs::read_dir(themes_dir)
        .map_err(|e| e.to_string())?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().unwrap() == "toml")
        .collect();

    return Ok(themes);
}

pub fn format_theme(theme_path: &PathBuf) -> &str {
    let filename = theme_path.file_stem();

    return match filename {
        Some(filename) => filename.to_str().unwrap(),
        None => theme_path.to_str().unwrap(),
    };
}
