use crate::{
    config::Config,
    helpers::{convert_unix_timestamp, get_unix_timestamp},
};
use colored::Colorize;
use inquire::MultiSelect;
use rusqlite::{params, Connection, types::FromSql};
use std::cmp::Reverse;
use std::fmt;
use clap::ValueEnum;

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Status {
    Done,
    Active,
    Pending
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
                }
                else if str_status == "active" {
                    Status::Active
                }
                else {
                    Status::Pending
                }
            },
                _ => Status::Pending
        })
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub status: Status,
    pub priority: i32,
    pub done_date: Option<u64>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Filter {
    All,
    Done,
    Pending
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::All => write!(f, "all"),
            Filter::Done => write!(f, "done"),
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
                },
                Status::Pending => {
                    format!("{}", task.status.to_string().bright_magenta())
                },
                Status::Active => {
                    format!("{}", task.status.to_string().bright_blue())
                },
            };

            let styled_text: String = match task.status {
                Status::Done => {
                    task.text.strikethrough().to_string()
                },
                Status::Pending => {
                    task.text.to_string()
                },
                Status::Active => {
                    task.text.to_string()
                },
            };

            let done_date: String = match task.done_date {
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

            let msg = format!(
                "{id} {date} {status} {text}",
                id = task.id.bright_black(),
                date = done_date,
                status = styled_is_done,
                text = styled_text
            );

            println!(
                "  {}",
                if task.status == Status::Done {
                    msg.bright_black().to_string()
                } else {
                    match task.priority {
                        2 => msg.bright_yellow().to_string(),
                        i if i >= 3 => msg.bright_red().to_string(),

                        _ => msg,
                    }
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

            let done_date: String = match task.done_date {
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
                Status::Done => {
                    task.text.strikethrough().to_string()
                },
                Status::Pending => {
                    task.text.to_string()
                },
                Status::Active => {
                    task.text.to_string()
                },
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
        let sql = match filter {
            Filter::All => "SELECT * FROM tasks",
            Filter::Done => "SELECT * FROM tasks WHERE is_done = 1",
            Filter::Pending => "SELECT * FROM tasks WHERE is_done = 0"
        };

        let mut stmt = self.conn.prepare(sql).unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                    done_date: row.get(5).unwrap_or(None),
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
            .query_row("SELECT * FROM tasks WHERE id = ?", [task_id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    priority: row.get(4)?,
                    done_date: row.get(5).unwrap_or(None),
                })
            })
            .unwrap()
    }

    /// Update task text
    pub fn update_text(&self, id: &String, text: String) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "UPDATE tasks SET text = ?1 WHERE id = ?2",
            [text, id.into()],
        )
    }

    pub fn update_status(&self, id: &String, status: Status) {
        match self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status.to_string(), id],
        ) {
            Ok(rows_updated) => {
                if rows_updated != 0 {
                    if status == Status::Done {
                        // if done
                        self.update_done_date(id);
                        self.update_priority(id, 0);
                    } else {
                        // if undone
                        self.update_priority(id, 1);
                    }

                    println!(
                        "{}",
                        format!("task {} is {}", id, if status == Status::Done { "done" } else { "undone" })
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

    pub fn update_done_date(&self, id: &String) {
        let time_stamp = get_unix_timestamp();

        match self.conn.execute(
            "UPDATE tasks SET done_date = ?1 WHERE id = ?2",
            params![time_stamp, id],
        ) {
            Ok(rows_updated) => {
                if rows_updated == 0 {
                    println!("failed to update done_date {time_stamp}");
                }
            }
            Err(err) => {
                println!("Failed: {}", err)
            }
        }
    }

    pub fn update_priority(&self, id: &String, n: i32) {
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
            "INSERT INTO tasks (id, category, text, status, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &new_task.id,
                &new_task.category,
                &new_task.text,
                &new_task.status.to_string(),
                &new_task.priority,
            ),
        )
    }

    pub fn move_task(&self, category: &String, id: &String) {
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
