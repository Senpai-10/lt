// use this https://docs.rs/sqlite/1.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod app;
mod args;
mod config;
mod db;
mod helpers;
mod editor;

use args::Args;
use clap::Parser;
use config::{get_config, Config, DEFAULT_CONFIG};
use rusqlite::{Connection, Result};
use std::process::exit;

fn main() -> Result<()> {
    let args = Args::parse();

    let config: Config = get_config();

    if args.print_default_config {
        println!("{}", DEFAULT_CONFIG);
        exit(0);
    }

    let db_file: String = match &args.file {
        Some(file) => file.to_string(),
        None => config.default_database_file.clone(),
    };

    let conn = Connection::open(db_file)?;

    db::setup(&conn);

    app::init(conn, args, config);

    Ok(())
}
