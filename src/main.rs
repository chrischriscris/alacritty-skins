use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml_edit::DocumentMut;

#[derive(Debug)]
enum SupportedPlatform {
    Unix,
    Windows,
}

fn detect_platform() -> Result<SupportedPlatform, String> {
    return match env::consts::OS {
        "linux" | "macos" => Ok(SupportedPlatform::Unix),
        "windows" => Ok(SupportedPlatform::Windows),
        other => Err(format!("Unsupported platform: {}", other)),
    };
}

/// Gets the path to the Alacritty configuration file for the given platform
fn get_config_file_path(platform: SupportedPlatform) -> Result<PathBuf, String> {
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
fn get_themes() -> Result<Vec<PathBuf>, String> {
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

fn format_theme(theme_path: &PathBuf) -> &str {
    let filename = theme_path.file_name();

    return match filename {
        Some(filename) => filename.to_str().unwrap(),
        None => theme_path.to_str().unwrap()
    }
}


fn try_main() -> Result<(), String> {
    let platform = detect_platform()?;
    let config_file = get_config_file_path(platform)?;
    let config_file = fs::read_to_string(config_file).expect("Unable to read file");

    let mut parsed = match config_file.parse::<DocumentMut>() {
        Ok(parsed) => parsed,
        Err(error) => return Err(format!("Failed to parse configuration file: {}", error)),
    };

    //  2.2 If it's not a toml return, only operate on toml

    // 3. Read the file, is it possible to keep it open?
    //    The idea is to have it continuosly open and write and save
    //    without closing it in a loop to make it a live preview

    // 4. Display a nice looking UI that lists all the available themes,
    //    lets you scroll through them, preview the colors and fuzzyfind some
    // println!("Select a theme: {:?}", platform);

    // 5. Add the theme to the config file
    let binding = get_themes()?;
    let themes: Vec<&str>  = binding.iter().map(|t| format_theme(t)).collect();

    println!("{:?}", themes);

    // 5. Return when escaping
    let imports = parsed["import"].as_array_mut().expect("Not an array ahaha");

    imports.push(format!(
        "~/Projects/alacritty-skins/themes/{}.toml",
        "gruvbox"
    ));

    // 6. Save the file
    fs::write("alacritty.toml", parsed.to_string()).expect("Unable to write file");

    println!("{}", parsed);

    Ok(())
}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
