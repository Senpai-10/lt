use crate::db;
use rusqlite::Connection;

pub fn run(conn: &Connection, ids: &Vec<String>) {
    for id in ids {
        let task = db::tasks::query_one(&conn, &id);

        let new_text = inquire::Text::new("update task:")
            .with_initial_value(&task.text)
            .prompt()
            .unwrap();

        match db::tasks::update_text(&conn, &id, new_text) {
            Ok(rows_updated) => {
                if rows_updated != 0 {
                    println!("task {}'s text is updated!", id)
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }
}
