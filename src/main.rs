// use this https://docs.rs/sqlite/0.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod cli;
mod db;
mod helpers;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use db::{get_all_tasks, get_task, print_tasks, Task};
use dotenv::dotenv;
use helpers::{generate_id, calculate_percentage};
use inquire;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::process::exit;

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
        (),
    )?;

    match &cli.commands {
        Some(Commands::Add { task, category }) => {
            let id = generate_id(4);

            let new_task = Task {
                id,
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

        Some(Commands::Delete { ids }) => {
            for id in ids {
                match conn.execute("DELETE FROM tasks WHERE id = ?", [id]) {
                    Ok(number_of_updated_row) => {
                        if number_of_updated_row != 0 {
                            println!("task {} is removed", id)
                        } else {
                            println!("no task with id '{}' is found!", id)
                        }
                    }
                    Err(err) => {
                        println!("Failed: {}", err)
                    }
                }
            }
        }

        Some(Commands::Edit { ids }) => {
            for id in ids {
                let task = get_task(&conn, id);

                let new_text = inquire::Text::new("update task:")
                    .with_initial_value(&task.text)
                    .prompt()
                    .unwrap();

                conn.execute(
                    "UPDATE tasks SET text = ?1 WHERE id = ?2",
                    [new_text, id.into()],
                )?;
            }
        }

        Some(Commands::List { category }) => {
            let tasks = get_all_tasks(&conn);

            let mut categories: HashMap<String, Vec<Task>> = HashMap::new();
            let mut done_count: HashMap<String, usize> = HashMap::new();
            let total_tasks: i32 = tasks.len() as i32;
            let mut total_done = 0;

            for task in tasks {
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
            }

            if total_tasks == 0 {
                println!("{}", format!("No tasks found!").bright_black());
                exit(0);
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

        Some(Commands::Done { ids }) => {
            for id in ids {
                match conn.execute(
                    "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
                    params![true, id],
                ) {
                    Ok(number_of_updated_row) => {
                        if number_of_updated_row > 0 {
                            println!("task {} is done", id)
                        }
                    }
                    Err(err) => {
                        println!("Failed: {}", err)
                    }
                }
            }
        }

        Some(Commands::Undone { ids }) => {
            for id in ids {
                match conn.execute(
                    "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
                    params![false, id],
                ) {
                    Ok(number_of_updated_row) => {
                        if number_of_updated_row > 0 {
                            println!("task {} is undone", id)
                        }
                    }
                    Err(err) => {
                        println!("Failed: {}", err)
                    }
                }
            }
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

