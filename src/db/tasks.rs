use crate::{
    config::Config,
    helpers::{convert_unix_timestamp, get_unix_timestamp},
};
use colored::Colorize;
use inquire::MultiSelect;
use rusqlite::{params, Connection};
use std::cmp::Reverse;

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub is_done: bool,
    pub priority: i32,
    pub done_date: Option<u64>,
}

pub struct TasksManager {
    pub conn: Connection,
    pub config: Config
}

impl TasksManager {
    pub fn new(conn: Connection, config: Config) -> Self {
        Self {
            conn,
            config
        }
    }

    pub fn print_all_tasks_to_console(category: &String, dones: &usize, tasks: &mut Vec<Task>, date_format: &String) -> () {}

}

pub fn print_all(category: &String, dones: &usize, tasks: &mut Vec<Task>, date_format: &String) {
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
        let styled_is_done: String = match task.is_done {
            true => {
                format!("{}", "".bright_green())
            }
            false => {
                format!("{}", "".bright_magenta())
            }
        };

        let styled_text: String = match task.is_done {
            true => task.text.strikethrough().to_string(),
            false => task.text.to_string(),
        };

        let done_date: String = match task.done_date {
            Some(unix_timestamp) => {
                if !task.is_done {
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
            if task.is_done {
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

pub fn interactive_multi_select(config: Config, tasks: &Vec<Task>) -> Vec<String> {
    let mut indices: Vec<String> = Vec::new();
    let mut options: Vec<String> = Vec::new();

    for task in tasks {
        let styled_is_done: String = match task.is_done {
            true => {
                format!("{}", "DONE")
            }
            false => {
                format!("{}", "PENDING")
            }
        };

        let done_date: String = match task.done_date {
            Some(unix_timestamp) => {
                if !task.is_done {
                    String::new()
                } else {
                    let date = convert_unix_timestamp(unix_timestamp, &config.date_format);

                    format!("{}", date)
                }
            }
            None => String::new(),
        };

        let msg = format!(
            "{id} {category} {date} {status} {text}",
            id = task.id,
            category = task.category,
            date = done_date,
            status = styled_is_done,
            text = task.text
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

pub fn query_all(conn: &Connection) -> Vec<Task> {
    let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                category: row.get(1)?,
                text: row.get(2)?,
                is_done: row.get(3)?,
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

pub fn query_one(conn: &Connection, task_id: &String) -> Task {
    conn.query_row("SELECT * FROM tasks WHERE id = ?", [task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            category: row.get(1)?,
            text: row.get(2)?,
            is_done: row.get(3)?,
            priority: row.get(4)?,
            done_date: row.get(5).unwrap_or(None),
        })
    })
    .unwrap()
}

pub fn update_text(conn: &Connection, id: &String, text: String) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE tasks SET text = ?1 WHERE id = ?2",
        [text, id.into()],
    )
}

pub fn update_is_done(conn: &Connection, id: &String, value: bool) {
    match conn.execute(
        "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
        params![value, id],
    ) {
        Ok(rows_updated) => {
            if rows_updated != 0 {
                if value == true {
                    // if done
                    update_done_date(conn, id);
                    update_priority(conn, id, 0);
                } else {
                    // if undone
                    update_priority(conn, id, 1);
                }

                println!(
                    "{}",
                    format!("task {} is {}", id, if value { "done" } else { "undone" })
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

pub fn update_done_date(conn: &Connection, id: &String) {
    let time_stamp = get_unix_timestamp();

    match conn.execute(
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

pub fn update_priority(conn: &Connection, id: &String, n: i32) {
    match conn.execute(
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

pub fn add_task(conn: &Connection, new_task: Task) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO tasks (id, category, text, is_done, priority) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &new_task.id,
            &new_task.category,
            &new_task.text,
            &new_task.is_done,
            &new_task.priority,
        ),
    )
}

pub fn move_task(conn: &Connection, category: &String, id: &String) {
    match conn.execute(
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

pub fn remove_task(conn: &Connection, id: &String) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM tasks WHERE id = ?", [id])
}

pub fn remove_all_tasks(conn: &Connection) {
    match conn.execute("DROP TABLE tasks", ()) {
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

pub fn remove_all_tasks_from(conn: &Connection, category: &String) {
    match conn.execute("DELETE FROM tasks WHERE category = ?1", params![category]) {
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
