use serde::Deserialize;
use directories::ProjectDirs;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub default_database_file: String,
    pub id_length: usize,
    pub date_format: String,
}

pub const DEFAULT_CONFIG: &str = r#"### Default Config ###

# This option is used
# if no --file provided
default_database_file = "todo.db"

# Length of genrated id for tasks
id_length = 3

# Format of date used when listing a task
date_format = "%Y-%m-%d %I:%M:%S %P"
"#;

pub fn get_config() -> Config {
    let proj_dirs = ProjectDirs::from("com", "senpai-10", "todo").unwrap();
    let config_dir = proj_dirs.config_dir();

    let config_file = fs::read_to_string(config_dir.join("config.toml"));

    match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => Config {
            default_database_file: "todo.db".to_string(),
            id_length: 3,
            date_format: "%Y-%m-%d %I:%M:%S %P".to_string(),
        },
    }
}
