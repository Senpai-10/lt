use rusqlite::Connection;
use crate::db;

pub fn run(conn: &Connection, ids: &Vec<String>) {
    for id in ids {
        db::tasks::update_is_done(&conn, &id, true)
    }
}
