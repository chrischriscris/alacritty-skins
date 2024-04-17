import shutil
from os.path import expanduser

import toml

USER_HOME = expanduser("~")
CONFIG_DIR = f"{USER_HOME}/.config/alacritty"
CONFIG_PATH = f"{CONFIG_DIR}/alacritty.toml"
NEW_THEME_PATH_BASE = f"{CONFIG_DIR}/themes/themes/{{}}.toml"

def main():
    config = toml.load(CONFIG_PATH)

    with open(f"{CONFIG_PATH}.bak", "w") as f:
        shutil.copyfileobj(open(CONFIG_PATH, "r"), f)

    while True:
        theme_name = input("Enter theme name (press 'q' to quit): ")
        if theme_name == "q":
            break

        config["import"][0] = NEW_THEME_PATH_BASE.format(theme_name)

        with open(CONFIG_PATH, "w") as f:
            toml.dump(config, f)

        print("Theme changed to", theme_name)


if __name__ == "__main__":
    main()
