use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::api::path;

#[derive(Deserialize, Serialize)]
struct Config {
    theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
        }
    }
}

fn get_config_dir_path() -> PathBuf {
    let lt_dir = path::config_dir().unwrap().join("lt");

    if !lt_dir.exists() {
        match std::fs::create_dir_all(&lt_dir) {
            Ok(_) => {
                println!("lt config directory created! {}", lt_dir.to_str().unwrap());
            }
            Err(e) => {
                eprintln!("Error creating lt config directory: {e}")
            }
        }
    }

    lt_dir
}

fn get_config() -> Config {
    let p = get_config_dir_path().join("config.toml");

    if !p.exists() {
        // Create file with default settings
        if let Ok(toml_string) = toml::to_string(&Config::default()) {
            return match std::fs::write(&p, toml_string) {
                Ok(_) => {
                    println!("Created `config.toml` with default settings!");
                    Config::default()
                }
                Err(e) => {
                    eprintln!("Failed to create `config.toml`! {e}");
                    Config::default()
                }
            };
        }
    }

    match std::fs::read_to_string(p) {
        Ok(v) => toml::from_str(&v).unwrap(),
        Err(_) => Config::default(),
    }
}

fn write_config(config: Config) {
    let p = get_config_dir_path().join("config.toml");

    if let Ok(toml_string) = toml::to_string(&config) {
        match std::fs::write(p, toml_string) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write config file: {e}");
            }
        }
    }
}

fn get_themes_dir_path() -> PathBuf {
    let themes_dir = path::config_dir().unwrap().join("lt").join("themes");

    if !themes_dir.exists() {
        match std::fs::create_dir_all(&themes_dir) {
            Ok(_) => {
                println!(
                    "lt themes directory created! {}",
                    themes_dir.to_str().unwrap()
                );
            }
            Err(e) => {
                eprintln!("Error creating lt themes directory: {e}")
            }
        }
    }

    themes_dir
}

#[tauri::command]
pub fn get_current_theme() -> String {
    let config = get_config();

    config.theme
}

#[tauri::command]
pub fn get_theme_css(target_theme: Option<String>) -> String {
    let theme_name = match target_theme {
        Some(n) => n,
        None => get_current_theme(),
    };

    if theme_name == "default" {
        return String::new();
    }

    let theme_file = get_themes_dir_path().join(format!("{}.css", theme_name));

    match std::fs::read_to_string(theme_file) {
        Ok(css_string) => css_string,
        Err(_) => String::new(),
    }
}

#[tauri::command]
pub fn set_theme(new_theme: String) {
    let mut config = get_config();

    config.theme = new_theme;

    write_config(config);
}

#[tauri::command]
pub fn get_available_themes() -> Vec<String> {
    let themes_dir = get_themes_dir_path();
    let mut themes: Vec<String> = vec!["default".to_string()];

    let themes_dir_content = match themes_dir.read_dir() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to read dir themes directory! {e}");

            return themes;
        }
    };

    for theme in themes_dir_content.flatten() {
        let theme_name = theme.file_name().to_str().unwrap().to_string();

        if !theme.path().is_file() || !theme_name.ends_with(".css") {
            continue;
        }

        let file_name_without_stem = theme
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        themes.push(file_name_without_stem)
    }

    themes
}
