// use this https://docs.rs/sqlite/1.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod apps;
mod args;
mod db;
mod helpers;

use std::fs;

use args::Args;
use clap::Parser;
use directories::ProjectDirs;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    default_database_file: String,
    id_length: usize,
    date_format: String,
}

fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let proj_dirs = ProjectDirs::from("com", "senpai-10", "todo").unwrap();
    let config_dir = proj_dirs.config_dir();

    let config_file = fs::read_to_string(config_dir.join("config.toml"));

    let config: Config = match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => Config {
            default_database_file: "todo.db".to_string(),
            id_length: 3,
            date_format: "%Y-%m-%d %I:%M:%S %P".to_string(),
        },
    };

    dbg!(&config);

    let db_file: String = match &args.file {
        Some(file) => file.to_string(),
        None => config.default_database_file,
    };

    let conn = Connection::open(db_file)?;

    db::setup(&conn)?;

    apps::cli::init(&conn, &args);

    Ok(())
}
