// use this https://docs.rs/sqlite/0.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod cli;
mod tasks;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use dotenv::dotenv;
use inquire;
use nanoid::nanoid;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::process::exit;
use tasks::{print_tasks, Task};

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    let db_file = match cli.file {
        Some(file) => file,
        None => String::from("todo.db"),
    };

    let conn = Connection::open(db_file)?;

    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS tasks (
                 id          VARCHAR     NOT NULL PRIMARY KEY,
                 category    VARCHAR     NOT NULL,
                 text        TEXT        NOT NULL,
                 is_done     BOOLEAN     NOT NULL DEFAULT 'false'
            )
        "#,
        (), // empty list of parameters.
    )?;

    match &cli.commands {
        Some(Commands::Add { task, category }) => {
            let id = nanoid!(8);

            let new_task = Task {
                id: id,
                category: category.into(),
                text: task.into(),
                is_done: false,
            };

            conn.execute(
                "INSERT INTO tasks (id, category, text, is_done) VALUES (?1, ?2, ?3, ?4)",
                (
                    &new_task.id,
                    &new_task.category,
                    &new_task.text,
                    &new_task.is_done,
                ),
            )?;
        }

        Some(Commands::Delete { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::Edit { task_id }) => {
            let task = conn.query_row(
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
            )?;

            let new_text = inquire::Text::new("update task").with_initial_value(&task.text).prompt().unwrap();

            conn.execute(
                "UPDATE tasks SET text = ?1 WHERE id = ?2",
                [new_text, task_id.into()]
            )?;
        }

        Some(Commands::List { category }) => {
            let mut stmt = conn.prepare("SELECT * FROM tasks")?;
            let tasks_iter = stmt.query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    is_done: row.get(3)?,
                })
            })?;

            let mut categories: HashMap<String, Vec<Task>> = HashMap::new();
            let mut done_count: HashMap<String, usize> = HashMap::new();
            let mut total_done = 0;
            let mut total_tasks = 0;

            for task in tasks_iter {
                let task = task.unwrap();
                let key = &task.category;

                if task.is_done {
                    let count = done_count.entry(key.into()).or_insert(0);
                    *count += 1;
                    total_done += 1;
                }

                categories
                    .entry(key.into())
                    .or_insert(Vec::new())
                    .push(task);

                total_tasks += 1;
            }

            match category {
                Some(category) => {
                    let tasks = categories.get(category);

                    match tasks {
                        Some(tasks) => {
                            let dones = done_count.get(category).unwrap_or(&(0 as usize));

                            print_tasks(category, dones, tasks);
                        }
                        None => {
                            println!("category '{}' is not found", category);
                            exit(1);
                        }
                    }
                }
                None => {
                    for key in categories.keys().into_iter() {
                        let tasks = categories.get(key).unwrap();
                        let dones = done_count.get(key).unwrap_or(&(0 as usize));

                        print_tasks(key, dones, tasks);
                    }

                    println!();

                    println!(
                        "{}",
                        format!(
                            "{}% of all tasks complete.",
                            calculate_percentage(total_done, total_tasks)
                        )
                        .bright_black()
                    );

                    println!(
                        "{}",
                        format!(
                            "{} done, {} undone",
                            total_done.to_string().bright_green(),
                            (total_tasks - total_done).to_string().bright_magenta()
                        )
                        .bright_black()
                    )
                }
            }
        }

        Some(Commands::Done { task_id }) => {
            conn.execute(
                "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
                params![true, task_id],
            )?;
        }

        Some(Commands::Undone { task_id }) => {
            conn.execute(
                "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
                params![false, task_id],
            )?;
        }

        Some(Commands::Clear {}) => {
            let confirm = inquire::Confirm::new("Are you sure you want to remove all tasks")
                .with_default(false)
                .prompt()
                .unwrap();

            if confirm == true {
                conn.execute("DROP TABLE tasks", ())?;
            }
        }

        None => {}
    }

    Ok(())
}

fn calculate_percentage(part: i32, whole: i32) -> i32 {
    100 * part / whole
}
