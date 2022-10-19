use rusqlite::Connection;
use crate::Task;

pub fn get_task(conn: &Connection, task_id: &String) -> Task {
    conn.query_row(
        "SELECT * FROM tasks WHERE id = ?",
        [task_id],
        |row| {
            Ok(
                Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    is_done: row.get(3)?,
                }
            )
        },
    ).unwrap()
}
