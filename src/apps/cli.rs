use crate::args::{Args, Commands};
use crate::config::Config;
use crate::db::tasks::{Status, Task, TasksManager};
use crate::helpers::calculate_percentage;
use crate::helpers::generate_id;
use colored::Colorize;
use inquire;
use rusqlite::Connection;
use std::collections::HashMap;
use std::process::exit;

pub fn init(conn: Connection, args: Args, config: Config) {
    let tasks_manager = TasksManager::new(conn, config);

    match args.commands {
        Some(Commands::Add {
            category,
            id_length,
            priority,
            task,
        }) => {
            let length: usize = match id_length {
                Some(len) => len,
                None => tasks_manager.config.id_length,
            };

            let id = generate_id(length);

            let new_task = Task {
                id,
                category: category.into(),
                text: task.into(),
                status: Status::Pending,
                priority,
                done_date: None,
            };

            match tasks_manager.add_task(new_task) {
                Ok(rows_updated) => {
                    if rows_updated != 0 {
                        println!("{}", "New task added!".bright_green().bold())
                    }
                }
                Err(err) => println!("Failed: {}", err),
            }
        }

        Some(Commands::Delete {
            ids,
            interactive,
            filter,
        }) => {
            let mut ids = ids;

            if interactive {
                let tasks = tasks_manager.query_all(filter);

                ids = tasks_manager.interactive_multi_select(&tasks);
            }

            for id in ids {
                match tasks_manager.remove_task(&id) {
                    Ok(number_of_updated_row) => {
                        if number_of_updated_row != 0 {
                            println!("task {} was removed", id.bright_blue().bold())
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
        Some(Commands::Edit {
            ids,
            interactive,
            filter,
        }) => {
            let ids = match interactive {
                true => {
                    let tasks = tasks_manager.query_all(filter);

                    tasks_manager.interactive_multi_select(&tasks)
                }
                false => ids,
            };

            for id in ids {
                let task = tasks_manager.query_one(&id);

                let new_text = inquire::Text::new("update task:")
                    .with_initial_value(&task.text)
                    .prompt()
                    .unwrap();

                match tasks_manager.update_text(&id, new_text) {
                    Ok(rows_updated) => {
                        if rows_updated != 0 {
                            println!("task {}'s text is updated!", id.bright_blue().bold())
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
            filter,
        }) => {
            let tasks = tasks_manager.query_all(filter);

            let format = match date_format {
                Some(format) => format,
                None => tasks_manager.config.date_format.to_string(),
            };

            let mut categories: HashMap<String, Vec<Task>> = HashMap::new();
            let mut done_count: HashMap<String, usize> = HashMap::new();
            let total_tasks: i32 = tasks.len() as i32;
            let mut total_done = 0;

            for task in tasks {
                let key = &task.category;

                if task.status == Status::Done {
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
                    let tasks = categories.get_mut(&category).unwrap_or_else(|| {
                        println!("category '{}' is not found", category);
                        exit(1);
                    });

                    let dones = done_count.get(&category).unwrap_or(&(0 as usize));

                    tasks_manager.print_all(&category, dones, tasks, &format);
                }
                None => {
                    for (key, tasks) in categories.iter_mut() {
                        let dones = done_count.get(key).unwrap_or(&(0 as usize));

                        tasks_manager.print_all(key, dones, tasks, &format);
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

        Some(Commands::Move {
            ids,
            category,
            interactive,
            filter,
        }) => {
            let ids = match interactive {
                true => {
                    let tasks = tasks_manager.query_all(filter);

                    tasks_manager.interactive_multi_select(&tasks)
                }
                false => ids,
            };

            for id in ids {
                tasks_manager.move_task(&category, &id);
            }
        }

        Some(Commands::Status {
            status,
            ids,
            interactive,
            filter,
        }) => {
            let ids = match interactive {
                true => {
                    let tasks = tasks_manager.query_all(filter);

                    tasks_manager.interactive_multi_select(&tasks)
                }
                false => ids,
            };

            for id in ids {
                tasks_manager.update_status(&id, status);
            }
        }

        Some(Commands::Clear { category }) => {
            let confirm = inquire::Confirm::new("Are you sure you want to remove tasks!")
                .with_default(false)
                .prompt()
                .unwrap();

            match category {
                Some(category) => {
                    if confirm == true {
                        tasks_manager.remove_all_tasks_from(&category);
                    }
                }
                None => {
                    if confirm == true {
                        tasks_manager.remove_all_tasks();
                    }
                }
            }
        }

        None => {}
    }
}
