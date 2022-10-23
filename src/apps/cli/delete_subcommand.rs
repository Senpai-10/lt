use crate::{db::{self, tasks}, config::Config};
use colored::Colorize;
use rusqlite::Connection;

pub fn run(conn: &Connection, config: Config, ids: Vec<String>, interactive: bool) {
    let mut ids = ids;

    if interactive {
        let tasks = tasks::query_all(conn);

        ids = tasks::interactive_multi_select(config, &tasks);
    }

    for id in ids {
            match db::tasks::remove_task(&conn, &id) {
                Ok(number_of_updated_row) => {
                    if number_of_updated_row != 0 {
                        println!("task {} was removed", id.bright_blue().bold())
                    } else {
                        println!("no task with id '{}' is found!", id)
                    }
                }
                Err(err) => {
                    println!("Failed: {}", err)
                }
            }
        }
}
