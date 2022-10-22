use rusqlite::Connection;
use crate::db;

pub fn run(conn: &Connection, ids: &Vec<String>) {
    for id in ids {
        match db::tasks::remove_task(&conn, &id) {
            Ok(number_of_updated_row) => {
                if number_of_updated_row != 0 {
                    println!("task {} was removed", id)
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
