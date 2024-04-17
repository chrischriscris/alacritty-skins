use std::env;

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

fn main() {
    // 1. Detect the OS
    let platform = match detect_platform() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    // 2. Find configuration file for the platform

    //  2.1 If file doesn't exists, create one
    //  2.2 If it's not a toml return, only operate on toml

    // 3. Read the file, is it possible to keep it open?
    //    The idea is to have it continuosly open and write and save
    //    without closing it in a loop to make it a live preview

    // 4. Display a nice looking UI that lists all the available themes,
    //    lets you scroll through them, preview the colors and fuzzyfind some
    println!("Select a theme: {:?}", platform);

    // 5. Return when escaping
}
