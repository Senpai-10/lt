use crate::{
    config::Config,
    helpers::{convert_unix_timestamp, get_unix_timestamp},
};
use clap::ValueEnum;
use colored::Colorize;
use inquire::MultiSelect;
use rusqlite::{params, types::FromSql, Connection};
use std::cmp::Reverse;
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
    // Creation,
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

    /// Print all tasks from a list with styles
    pub fn print_all(
        &self,
        category: &String,
        dones: &usize,
        tasks: &mut Vec<Task>,
        date_format: &String,
    ) -> () {
        tasks.sort_by_key(|k| Reverse(k.priority));

        let mut count = format!("[{}/{}]", dones, tasks.len());

        // color if category is in progress
        if *dones > 0 && *dones != tasks.len() {
            count = count.bright_yellow().bold().to_string()
        }

        // color if all tasks in category are done
        if *dones == tasks.len() {
            count = count.bright_green().bold().to_string()
        }

        println!(
            "\n{} {}",
            format!(" {} ", category).on_bright_cyan().black().bold(),
            count
        );

        for task in tasks {
            let styled_is_done: String = match task.status {
                Status::Done => {
                    format!("{}", task.status.to_string().bright_green())
                }
                Status::Pending => {
                    format!("{}", task.status.to_string().bright_magenta())
                }
                Status::Active => {
                    format!("{}", task.status.to_string().bright_blue())
                }
            };

            let text = task.text.replace("\n", "\n\t");

            let styled_text: String = match task.status {
                Status::Done => text.bright_black().strikethrough().to_string(),
                Status::Pending => text.to_string(),
                Status::Active => text.to_string(),
            };

            let done_date: String = match task.completion_date {
                Some(unix_timestamp) => {
                    if task.status != Status::Done {
                        String::new()
                    } else {
                        let date = convert_unix_timestamp(unix_timestamp, date_format);

                        format!("{}", date.bright_green().underline())
                    }
                }
                None => String::new(),
            };

            // @category
            // <TASK_ID> <STATUS> <PRIORITY> / <CREATION_DATE: DATE> <COMPLATION_DATE: DATE> <LASTMODIFCTION_DATE: DATE>
            // testtext text this is a task! test test test
            //      test test test test test test test test test
            //      test test test test
            let msg = format!(
                "{id} {status} {priority} (creation date: {creation_date}, complation date: {complation_date}, last modifction: {lastmodifction_date})\n\t{text}",
                id = task.id.bright_black(),
                status = styled_is_done,
                priority = task.priority,
                creation_date = task.creation_date,
                complation_date = done_date,
                lastmodifction_date = task.modification_date,
                text = styled_text
            );

            println!(
                "  {}",
                match task.priority {
                    2 => msg.bright_yellow().to_string(),
                    i if i >= 3 => msg.bright_red().to_string(),

                    _ => msg,
                }
            );
        }
    }

    /// MultiSelect From a Vec
    pub fn interactive_multi_select(&self, tasks: &Vec<Task>) -> Vec<String> {
        let mut indices: Vec<String> = Vec::new();
        let mut options: Vec<String> = Vec::new();

        for task in tasks {
            let styled_is_done: String = match task.status {
                Status::Done => format!("{}", "DONE").bright_green().to_string(),
                Status::Pending => format!("{}", "PENDING").bright_magenta().to_string(),
                Status::Active => format!("{}", "ACTIVE").bright_blue().to_string(),
            };

            let done_date: String = match task.completion_date {
                Some(unix_timestamp) => {
                    if task.status == Status::Done {
                        let date = convert_unix_timestamp(unix_timestamp, &self.config.date_format);

                        format!("{} ", date)
                    } else {
                        String::new()
                    }
                }
                None => String::new(),
            };

            let styled_text: String = match task.status {
                Status::Done => task.text.strikethrough().to_string(),
                Status::Pending => task.text.to_string(),
                Status::Active => task.text.to_string(),
            };

            let msg = format!(
                "{id} {category} {status} {date}{text}",
                id = task.id.bright_black(),
                category = format!("@{}", task.category).bright_cyan(),
                status = styled_is_done,
                date = done_date.bright_green().underline(),
                text = styled_text
            );

            let formated = format!("{}", msg);

            indices.push(task.id.clone());
            options.push(formated);
        }

        let selected_options = MultiSelect::new("Select tasks:", options.clone())
            .with_vim_mode(true)
            .prompt();
        let mut selected: Vec<String> = Vec::new();

        match selected_options {
            Ok(items) => {
                for item in items {
                    let selected_index = options.iter().position(|x| *x == item).unwrap();
                    selected.push(indices.get(selected_index).unwrap().clone());
                }
            }
            Err(_) => println!("The tasks list could not be processed"),
        }

        selected
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

    pub fn move_task(&self, category: &String, id: &String) {
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
