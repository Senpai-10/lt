use crate::db;
use inquire;
use rusqlite::Connection;

pub fn run(conn: &Connection, category: &Option<String>) {
    let confirm = inquire::Confirm::new("Are you sure you want to remove tasks!")
                .with_default(false)
                .prompt()
                .unwrap();

    match category {
        Some(category) => {
            if confirm == true {
                db::tasks::remove_all_tasks_from(&conn, category);
            }
        }
        None => {
            if confirm == true {
                db::tasks::remove_all_tasks(&conn);
            }
        }
    }
}
