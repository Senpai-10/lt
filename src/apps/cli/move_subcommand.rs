use crate::db;
use rusqlite::Connection;

pub fn run(conn: &Connection, category: &String, ids: &Vec<String>) {
    for id in ids {
        db::tasks::move_task(&conn, &category, &id);
    }
}
