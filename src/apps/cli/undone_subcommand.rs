use crate::config::Config;
use crate::db;
use rusqlite::Connection;

pub fn run(conn: &Connection, config: Config, ids: Vec<String>, interactive: bool) {
    let ids = match interactive {
        true => {
            let tasks = db::tasks::query_all(conn);

            db::tasks::interactive_multi_select(config, &tasks)
        }
        false => ids
    };

    for id in ids {
        db::tasks::update_is_done(&conn, &id, false)
    }
}
