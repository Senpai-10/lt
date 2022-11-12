use crate::{config::Config, helpers::get_unix_timestamp};
use clap::ValueEnum;
use colored::Colorize;
use rusqlite::{params, types::FromSql, Connection};
use std::fmt;

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Status {
    Done,
    Active,
    Pending,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Done => write!(f, "done"),
            Status::Active => write!(f, "active"),
            Status::Pending => write!(f, "pending"),
        }
    }
}

impl FromSql for Status {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        Ok(match value {
            rusqlite::types::ValueRef::Text(v) => {
                let str_status = std::str::from_utf8(v).unwrap();

                if str_status == "done" {
                    Status::Done
                } else if str_status == "active" {
                    Status::Active
                } else {
                    Status::Pending
                }
            }
            _ => Status::Pending,
        })
    }
}

pub enum UpdateDate {
    Completion,
    Modification,
}

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub status: Status,
    pub priority: i32,
    pub creation_date: u64,
    pub completion_date: Option<u64>,
    pub modification_date: u64,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Filter {
    All,
    Done,
    Active,
    Pending,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::All => write!(f, "all"),
            Filter::Done => write!(f, "done"),
            Filter::Active => write!(f, "active"),
            Filter::Pending => write!(f, "pending"),
        }
    }
}

pub struct TasksManager {
    pub conn: Connection,
    pub config: Config,
}

impl TasksManager {
    pub fn new(conn: Connection, config: Config) -> Self {
        Self { conn, config }
    }

    pub fn query_all(&self, filter: Filter) -> Vec<Task> {
        let mut sql: String = String::from(
            r#"
            SELECT
                id,
                category,
                text,
                status,
                priority,
                creation_date,
                completion_date,
                modification_date
            FROM tasks
            "#,
        );

        match filter {
            Filter::Done => sql.push_str(" WHERE status = 'done'"),
            Filter::Active => sql.push_str(" WHERE status = 'active'"),
            Filter::Pending => sql.push_str(" WHERE status = 'pending'"),
            _ => {}
        }

        let mut stmt = self.conn.prepare(&sql).unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                    creation_date: row.get(5)?,
                    completion_date: row.get(5).unwrap_or(None),
                    modification_date: row.get(5)?,
                })
            })
            .unwrap();

        let mut tasks: Vec<Task> = Vec::new();

        for row in rows {
            let task = row.unwrap();

            tasks.push(task);
        }

        tasks
    }

    pub fn query_one(&self, task_id: &String) -> Task {
        self.conn
            .query_row("SELECT id, category, text, status, priority, creation_date, completion_date, modification_date FROM tasks WHERE id = ?", [task_id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                    creation_date: row.get(5)?,
                    completion_date: row.get(5).unwrap_or(None),
                    modification_date: row.get(5)?,
                })
            })
            .unwrap()
    }

    /// Update task text
    pub fn update_text(&self, id: &String, text: String) -> Result<usize, rusqlite::Error> {
        self.update_date(id, UpdateDate::Modification);

        self.conn.execute(
            "UPDATE tasks SET text = ?1 WHERE id = ?2",
            [text, id.into()],
        )
    }

    pub fn update_status(&self, id: &String, status: Status) {
        self.update_date(id, UpdateDate::Modification);

        match self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status.to_string(), id],
        ) {
            Ok(rows_updated) => {
                if rows_updated != 0 {
                    if status == Status::Done {
                        // if done
                        self.update_date(id, UpdateDate::Completion);
                        self.update_priority(id, 0);
                    } else {
                        // if undone
                        self.update_priority(id, 1);
                    }

                    println!(
                        "{}",
                        format!(
                            "task {} is {}",
                            id,
                            if status == Status::Done {
                                "done"
                            } else {
                                "undone"
                            }
                        )
                        .bright_green()
                        .bold()
                    )
                } else {
                    println!("no task with id '{}' is found!", id)
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn update_date(&self, id: &String, date_type: UpdateDate) {
        let time_stamp = get_unix_timestamp();

        let sql = match date_type {
            // UpdateDate::Creation => "UPDATE tasks SET creation_date = ?1 WHERE id = ?2",
            UpdateDate::Completion => "UPDATE tasks SET completion_date = ?1 WHERE id = ?2",
            UpdateDate::Modification => "UPDATE tasks SET modification_date = ?1 WHERE id = ?2",
        };

        match self.conn.execute(sql, params![time_stamp, id]) {
            Ok(rows_updated) => {
                if rows_updated == 0 {
                    println!("failed to update date {time_stamp}");
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn update_priority(&self, id: &String, n: i32) {
        self.update_date(id, UpdateDate::Modification);

        match self.conn.execute(
            "UPDATE tasks SET priority = ?1 WHERE id = ?2",
            params![n, id],
        ) {
            Ok(rows_updated) => {
                if rows_updated == 0 {
                    println!("failed to set priority to {n}");
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn add_task(&self, new_task: Task) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO tasks (id, category, text, status, priority, creation_date, modification_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &new_task.id,
                &new_task.category,
                &new_task.text,
                &new_task.status.to_string(),
                &new_task.priority,
                &new_task.creation_date,
                &new_task.modification_date
            ),
        )
    }

    pub fn update_category(&self, category: &String, id: &String) {
        self.update_date(id, UpdateDate::Modification);

        match self.conn.execute(
            "UPDATE tasks SET category = ?1 WHERE id = ?2",
            params![category, id],
        ) {
            Ok(rows_updated) => match rows_updated {
                0 => println!("no task with id '{}' is found!", id),
                1 => println!(
                    "{}",
                    format!("task {id} moved to {category}")
                        .bright_yellow()
                        .bold()
                ),
                _ => {}
            },
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn remove_task(&self, id: &String) -> Result<usize, rusqlite::Error> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?", [id])
    }

    pub fn remove_all_tasks(&self) {
        match self.conn.execute("DROP TABLE tasks", ()) {
            Ok(rows_updated) => match rows_updated {
                0 => println!("All tasks removed!"),
                1 => println!("failed to remove all tasks!"),
                _ => {}
            },
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn remove_all_tasks_from(&self, category: &String) {
        match self
            .conn
            .execute("DELETE FROM tasks WHERE category = ?1", params![category])
        {
            Ok(rows_updated) => match rows_updated {
                0 => println!("category '{category}' does not exists"),
                1 => println!("Removed all tasks from {category}!"),
                _ => {}
            },
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }
}
