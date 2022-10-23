// use this https://docs.rs/sqlite/1.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod apps;
mod args;
mod config;
mod db;
mod helpers;

use std::process::exit;
use args::Args;
use clap::Parser;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use config::{get_config, Config, DEFAULT_CONFIG};

fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let config: Config = get_config();

    if args.print_default_config {
        println!("{}", DEFAULT_CONFIG);
        exit(0);
    }

    let db_file: String = match &args.file {
        Some(file) => file.to_string(),
        None => config.default_database_file,
    };

    let conn = Connection::open(db_file)?;

    db::setup(&conn)?;

    apps::cli::init(&conn, &args);

    Ok(())
}
