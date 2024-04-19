use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml_edit::DocumentMut;

/// Supported platforms for Alacritty, along with an ordered list of possible
/// configuration file locations.
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

/// Opens the Alacritty configuration file for the given platform and returns
/// a string with its content
fn get_config_file_content(platform: SupportedPlatform) -> Result<String, String> {
    let mut config_files: Vec<PathBuf> = Vec::new();

    if let SupportedPlatform::Unix = platform {
        // $XDG_CONFIG_HOME/alacritty/alacritty.toml
        // $XDG_CONFIG_HOME/alacritty.toml
        // $HOME/.config/alacritty/alacritty.toml
        // $HOME/.alacritty.toml
        let _ = env::var("XDG_CONFIG_HOME").inspect(|val| {
            config_files.push(PathBuf::from(val.to_owned() + "/alacritty/alacritty.toml"));
            config_files.push(PathBuf::from(val.to_owned() + "/alacritty.toml"));
        });

        let _ = env::var("HOME").inspect(|val| {
            config_files.push(PathBuf::from(
                val.to_owned() + "/.config/alacritty/alacritty.toml",
            ));
            config_files.push(PathBuf::from(val.to_owned() + "/.alacritty.toml"));
        });
    } else if let SupportedPlatform::Windows = platform {
        // %APPDATA$\alacritty\alacritty.toml
        let _ = env::var("APPDATA").inspect(|val| {
            config_files.push(PathBuf::from(
                val.to_owned() + "\\alacritty\\alacritty.toml",
            ));
        });
    }

    for config_file in config_files {
        if Path::exists(&config_file) {
            return fs::read_to_string(config_file).map_err(|e| e.to_string());
        }
    }

    Err(String::from("Could not find configuration file"))
}

fn main() {
    let platform = match detect_platform() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let config = match get_config_file_content(platform) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    // Open the file and read it to a string
    let mut parsed = config.parse::<DocumentMut>().expect("Invalid config file");

    let imports = parsed["import"].as_array_mut().expect("Not an array ahaha");

    imports.push("~/path/to/a/theme");

    println!("{}", parsed);

    //  2.2 If it's not a toml return, only operate on toml

    // 3. Read the file, is it possible to keep it open?
    //    The idea is to have it continuosly open and write and save
    //    without closing it in a loop to make it a live preview

    // 4. Display a nice looking UI that lists all the available themes,
    //    lets you scroll through them, preview the colors and fuzzyfind some
    //println!("Select a theme: {:?}", platform);

    // 5. Return when escaping
}
