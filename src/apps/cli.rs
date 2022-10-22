mod add_subcommand;
mod delete_subcommand;
mod done_subcommand;
mod edit_subcommand;
mod list_subcommand;
mod move_subcommand;
mod undone_subcommand;
mod clear_subcommand;

use crate::args::{Args, Commands};
use rusqlite::Connection;

pub fn init(conn: &Connection, args: &Args) {
    match &args.commands {
        Some(Commands::Add {
            category,
            id_length,
            priority,
            task,
        }) => add_subcommand::run(conn, category, id_length, priority, task),

        Some(Commands::Delete { ids }) => delete_subcommand::run(conn, ids),
        Some(Commands::Edit { ids }) => edit_subcommand::run(conn, ids),

        Some(Commands::List {
            category,
            date_format,
        }) => list_subcommand::run(conn, category, date_format),

        Some(Commands::Move { ids, category }) => move_subcommand::run(conn, category, ids),
        Some(Commands::Done { ids }) => done_subcommand::run(conn, ids),
        Some(Commands::Undone { ids }) => undone_subcommand::run(conn, ids),
        Some(Commands::Clear { category }) => clear_subcommand::run(conn, category),

        None => {}
    }
}
