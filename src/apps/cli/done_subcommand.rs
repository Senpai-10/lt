use crate::db;
use rusqlite::Connection;

pub fn run(conn: &Connection, ids: &Vec<String>) {
    for id in ids {
        db::tasks::update_is_done(&conn, &id, true)
    }
}
