use rusqlite::Connection;
use crate::db;

pub fn run(conn: &Connection, category: &String, ids: &Vec<String>) {
    for id in ids {
        db::tasks::move_task(&conn, &category, &id);
    }
}
