use crate::db;
use rusqlite::Connection;
use inquire;

pub fn run(conn: &Connection) {
    let confirm = inquire::Confirm::new("Are you sure you want to remove all tasks")
        .with_default(false)
        .prompt()
        .unwrap();

    if confirm == true {
        db::tasks::remove_all_tasks(&conn);
    }
}
