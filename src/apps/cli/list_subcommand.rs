use crate::config::Config;
use crate::db;
use crate::helpers::calculate_percentage;
use colored::Colorize;
use rusqlite::Connection;
use std::collections::HashMap;
use std::process::exit;

pub fn run(conn: &Connection, config: Config, category: &Option<String>, date_format: &Option<String>) {
    let tasks = db::tasks::query_all(&conn);

    let format = match date_format {
        Some(format) => format,
        None => &config.date_format
    };

    let mut categories: HashMap<String, Vec<db::tasks::Task>> = HashMap::new();
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

            db::tasks::print_all(category, dones, tasks, format);
        }
        None => {
            for (key, tasks) in categories.iter_mut() {
                let dones = done_count.get(key).unwrap_or(&(0 as usize));

                db::tasks::print_all(key, dones, tasks, format);
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
