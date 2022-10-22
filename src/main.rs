// use this https://docs.rs/sqlite/0.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod args;
mod db;
mod helpers;
mod apps;

use clap::Parser;
use args::Args;
use dotenv::dotenv;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let db_file: String = match &args.file {
        Some(file) => file.to_string(),
        None => String::from("todo.db"),
    };

    let conn = Connection::open(db_file)?;

    db::setup(&conn)?;

    apps::cli::run(&conn, &args);

    Ok(())
}
