use crate::db;
use rusqlite::Connection;
use crate::config::Config;

pub fn run(conn: &Connection, config: Config, category: &String, ids: Vec<String>, interactive: bool) {
    let ids = match interactive {
        true => {
            let tasks = db::tasks::query_all(conn);

            db::tasks::interactive_multi_select(config, &tasks)
        }
        false => ids
    };

    for id in ids {
        db::tasks::move_task(&conn, &category, &id);
    }
}
