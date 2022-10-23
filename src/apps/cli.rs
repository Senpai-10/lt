mod add_subcommand;
mod clear_subcommand;
mod delete_subcommand;
mod done_subcommand;
mod edit_subcommand;
mod list_subcommand;
mod move_subcommand;
mod undone_subcommand;

use crate::{
    args::{Args, Commands},
    config::Config,
};
use rusqlite::Connection;

pub fn init(conn: &Connection, args: Args, config: Config) {
    match &args.commands {
        Some(Commands::Add {
            category,
            id_length,
            priority,
            task,
        }) => add_subcommand::run(conn, config, category, id_length, priority, task),

        Some(Commands::Delete { ids, interactive }) => delete_subcommand::run(conn, config, ids.to_vec(), interactive.to_owned()),
        Some(Commands::Edit { ids, interactive }) => edit_subcommand::run(conn, config, ids.to_vec(), interactive.to_owned()),

        Some(Commands::List {
            category,
            date_format,
        }) => list_subcommand::run(conn, config, category, date_format),

        Some(Commands::Move { ids, category, interactive }) => move_subcommand::run(conn, config, category, ids.to_owned(), interactive.to_owned()),
        Some(Commands::Done { ids, interactive }) => done_subcommand::run(conn, config, ids.to_vec(), interactive.to_owned()),
        Some(Commands::Undone { ids, interactive }) => undone_subcommand::run(conn, config, ids.to_vec(), interactive.to_owned()),
        Some(Commands::Clear { category }) => clear_subcommand::run(conn, category),

        None => {}
    }
}
