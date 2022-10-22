use crate::db;
use crate::helpers::generate_id;
use colored::Colorize;
use rusqlite::Connection;

pub fn run(conn: &Connection, category: &String, id_length: &usize, priority: &i32, task: &String) {
    let id = generate_id(*id_length);

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
