// use this https://docs.rs/sqlite/0.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod cli;
mod db;
mod helpers;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use db::tasks;
use dotenv::dotenv;
use helpers::{calculate_percentage, generate_id};
use inquire;
use rusqlite::{Connection, Result};
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

    db::setup(&conn)?;

    match &cli.commands {
        Some(Commands::Add { category , id_length, task }) => {
            let id = generate_id(*id_length);

            let new_task = tasks::Task {
                id,
                category: category.into(),
                text: task.into(),
                is_done: false,
            };

            match tasks::add_task(&conn, new_task) {
                Ok(rows_updated) => {
                    if rows_updated != 0 {
                        println!("New task added!")
                    }
                }
                Err(err) => println!("Failed: {}", err),
            }
        }

        Some(Commands::Delete { ids }) => {
            for id in ids {
                match tasks::remove_task(&conn, id) {
                    Ok(number_of_updated_row) => {
                        if number_of_updated_row != 0 {
                            println!("task {} was removed", id)
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
                let task = tasks::query_one(&conn, id);

                let new_text = inquire::Text::new("update task:")
                    .with_initial_value(&task.text)
                    .prompt()
                    .unwrap();

                match tasks::update_text(&conn, id.into(), new_text) {
                    Ok(rows_updated) => {
                        if rows_updated != 0 {
                            println!("task {}'s text is updated!", id)
                        }
                    }
                    Err(err) => {
                        println!("Failed: {}", err)
                    }
                }
            }
        }

        Some(Commands::List { category }) => {
            let tasks = tasks::query_all(&conn);

            let mut categories: HashMap<String, Vec<tasks::Task>> = HashMap::new();
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

                            tasks::print_all(category, dones, tasks);
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

                        tasks::print_all(key, dones, tasks);
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

        Some(Commands::Move { ids, category }) => {
            println!("ids: {:?}", ids);
            println!("category: {}", category);

            for id in ids {
                tasks::move_task(&conn, category, id);
            }
        }

        Some(Commands::Done { ids }) => {
            for id in ids {
                match tasks::update_is_done(&conn, id, true) {
                    Ok(rows_updated) => {
                        if rows_updated != 0 {
                            println!("task {} is done", id)
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

        Some(Commands::Undone { ids }) => {
            for id in ids {
                match tasks::update_is_done(&conn, id, false) {
                    Ok(rows_updated) => {
                        if rows_updated != 0 {
                            println!("task {} is undone", id)
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

        Some(Commands::Clear {}) => {
            let confirm = inquire::Confirm::new("Are you sure you want to remove all tasks")
                .with_default(false)
                .prompt()
                .unwrap();

            if confirm == true {
                tasks::remove_all_tasks(&conn);
            }
        }

        None => {}
    }

    Ok(())
}
