use crate::config::Config;
use crate::db;
use crate::helpers::generate_id;
use colored::Colorize;
use rusqlite::Connection;

pub fn run(conn: &Connection, config: Config, category: &String, id_length: &Option<usize>, priority: &i32, task: &String) {
    let length: usize = match id_length {
        Some(len) => *len,
        None => config.id_length
    };

    let id = generate_id(length);

    let new_task = db::tasks::Task {
        id,
        category: category.into(),
        text: task.into(),
        is_done: false,
        priority: *priority,
        done_date: None,
    };

    match db::tasks::add_task(&conn, new_task) {
        Ok(rows_updated) => {
            if rows_updated != 0 {
                println!("{}", "New task added!".bright_green().bold())
            }
        }
        Err(err) => println!("Failed: {}", err),
    }
}
