use crate::args::{Args, Commands};
use crate::config::Config;
use crate::db::tasks::{Status, Task, TasksManager};
use crate::editor;
use crate::helpers::{
    calculate_percentage, convert_unix_timestamp, generate_id, get_unix_timestamp,
    truncate_with_suffix,
};
use colored::Colorize;
use inquire;
use inquire::MultiSelect;
use rusqlite::Connection;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::process::exit;

pub fn init(conn: Connection, args: Args, config: Config) {
    let tasks_manager = TasksManager::new(conn, config);

    match args.commands {
        Some(Commands::Add {
            category,
            id_length,
            priority,
            title,
            text,
        }) => {
            let length: usize = match id_length {
                Some(len) => len,
                None => tasks_manager.config.id_length,
            };

            let id = generate_id(length);

            let text: String = match text {
                Some(v) => v.into(),
                None => {
                    let editor = env::var("EDITOR").unwrap_or("vim".into());

                    let confirm = inquire::Confirm::new(&format!(
                        "No Text provided, Do you want to enter task({}) text with your editor `{}`",
                        id, editor
                    )).with_default(true)
                        .prompt()
                        .unwrap();

                    if confirm == false {
                        return;
                    }

                    editor::edit(&id, editor, "".into())
                }
            };

            let new_task = Task {
                id,
                category,
                title,
                text,
                status: Status::Pending,
                priority,
                creation_date: get_unix_timestamp(),
                completion_date: None,
                modification_date: get_unix_timestamp(),
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

                ids = interactive_multi_select(&tasks, &tasks_manager.config.date_format);
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

                    interactive_multi_select(&tasks, &tasks_manager.config.date_format)
                }
                false => ids,
            };

            for id in ids {
                let task = tasks_manager.query_one(&id);
                let editor = env::var("EDITOR").unwrap_or("vim".into());

                let confirm = inquire::Confirm::new(&format!(
                    "Are you sure you want to edit task `{}` with `{}`",
                    id, editor
                ))
                .with_default(true)
                .prompt()
                .unwrap();

                if confirm == false {
                    continue;
                }

                let new_text = editor::edit(&id, editor, task.text);

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
            max_title_length,
            max_text_length,
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

                    print_all(&category, dones, tasks, &format, max_title_length, max_text_length);
                }
                None => {
                    for (key, tasks) in categories.iter_mut() {
                        let dones = done_count.get(key).unwrap_or(&(0 as usize));

                        print_all(key, dones, tasks, &format, max_title_length, max_text_length);
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

                    interactive_multi_select(&tasks, &tasks_manager.config.date_format)
                }
                false => ids,
            };

            for id in ids {
                tasks_manager.update_category(&category, &id);
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

                    interactive_multi_select(&tasks, &tasks_manager.config.date_format)
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

/// Print all tasks from a list with styles
fn print_all(
    category: &String,
    dones: &usize,
    tasks: &mut Vec<Task>,
    date_format: &String,
    max_title_length: usize,
    max_text_length: usize,
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

        let mut styled_text: String = match task.status {
            Status::Done => text.bright_black().strikethrough().to_string(),
            Status::Pending => text.bright_black().to_string(),
            Status::Active => text.bright_black().to_string(),
        };

        let creation_date: String = {
            let date = convert_unix_timestamp(task.creation_date, date_format);

            format!("{}", date.bright_green().underline())
        };

        let modification_date: String = {
            let date = convert_unix_timestamp(task.modification_date, date_format);

            format!("{}", date.bright_green().underline())
        };

        let complation_date: String = match task.completion_date {
            Some(unix_timestamp) => {
                if task.status != Status::Done {
                    String::from("NOT DONE")
                } else {
                    let date = convert_unix_timestamp(unix_timestamp, date_format);

                    format!("{}", date.bright_green().underline())
                }
            }
            None => String::new(),
        };

        truncate_with_suffix(&mut task.title, max_title_length, "...".bright_yellow().bold());
        truncate_with_suffix(&mut styled_text, max_text_length, "...".bright_yellow().bold());

        let msg = format!(
                "{id} {status} {priority} (creation: {creation_date}, complation: {complation_date}, last modifction: {lastmodifction_date})\n\t{title}\n\t{text}",
                id = task.id.bright_black(),
                status = styled_is_done,
                priority = task.priority,
                creation_date = creation_date,
                complation_date = complation_date,
                lastmodifction_date = modification_date,
                title = task.title.bold(),
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
fn interactive_multi_select(tasks: &Vec<Task>, date_format: &String) -> Vec<String> {
    let mut indices: Vec<String> = Vec::new();
    let mut options: Vec<String> = Vec::new();

    for task in tasks {
        let styled_status: String = match task.status {
            Status::Done => format!("{}", "DONE").bright_green().to_string(),
            Status::Pending => format!("{}", "PENDING").bright_magenta().to_string(),
            Status::Active => format!("{}", "ACTIVE").bright_blue().to_string(),
        };

        let done_date: String = match task.completion_date {
            Some(unix_timestamp) => {
                if task.status == Status::Done {
                    let date = convert_unix_timestamp(unix_timestamp, date_format);

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

        // TODO: cut off task.text and task.title off after 8 chars
        let msg = format!(
            "{id} {category} {status} {date}{title} {text}",
            id = task.id.bright_black(),
            category = format!("@{}", task.category).bright_cyan(),
            status = styled_status,
            date = done_date.bright_green().underline(),
            title = task.title.bold(),
            text = styled_text.bright_black()
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
