use crate::db;
use colored::Colorize;
use rusqlite::Connection;
use crate::config::Config;

pub fn run(conn: &Connection, config: Config, ids: Vec<String>, interactive: bool) {
    let ids = match interactive {
        true => {
            let tasks = db::tasks::query_all(conn);

            db::tasks::interactive_multi_select(config, &tasks)
        }
        false => ids
    };

    for id in ids {
        let task = db::tasks::query_one(&conn, &id);

        let new_text = inquire::Text::new("update task:")
            .with_initial_value(&task.text)
            .prompt()
            .unwrap();

        match db::tasks::update_text(&conn, &id, new_text) {
            Ok(rows_updated) => {
                if rows_updated != 0 {
                    println!("task {}'s text is updated!", id.bright_blue().bold())
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }
}
