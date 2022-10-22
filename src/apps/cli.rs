use rusqlite::Connection;
use colored::Colorize;
use std::collections::HashMap;
use std::process::exit;
use inquire;

use crate::helpers::{calculate_percentage, generate_id};
use crate::db::tasks;
use crate::args::{Args, Commands};

pub fn run(conn: &Connection, args: &Args) {
    match &args.commands {
        Some(Commands::Add {
            category,
            id_length,
            priority,
            task,
        }) => {
            let id = generate_id(*id_length);

            let new_task = tasks::Task {
                id,
                category: category.into(),
                text: task.into(),
                is_done: false,
                priority: *priority,
                done_date: None,
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
                match tasks::remove_task(&conn, &id) {
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
                let task = tasks::query_one(&conn, &id);

                let new_text = inquire::Text::new("update task:")
                    .with_initial_value(&task.text)
                    .prompt()
                    .unwrap();

                match tasks::update_text(&conn, &id, new_text) {
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

        Some(Commands::List {
            category,
            date_format,
        }) => {
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
                    let tasks = categories.get_mut(category).unwrap_or_else(|| {
                        println!("category '{}' is not found", category);
                        exit(1);
                    });

                    let dones = done_count.get(category).unwrap_or(&(0 as usize));

                    tasks::print_all(category, dones, tasks, date_format);
                }
                None => {
                    for (key, tasks) in categories.iter_mut() {
                        let dones = done_count.get(key).unwrap_or(&(0 as usize));

                        tasks::print_all(key, dones, tasks, date_format);
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
            for id in ids {
                tasks::move_task(&conn, &category, &id);
            }
        }

        Some(Commands::Done { ids }) => {
            for id in ids {
                tasks::update_is_done(&conn, &id, true)
            }
        }

        Some(Commands::Undone { ids }) => {
            for id in ids {
                tasks::update_is_done(&conn, &id, false)
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
}
