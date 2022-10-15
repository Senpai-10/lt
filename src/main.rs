mod cli;
mod filesystem;

use cli::{Cli, Commands};
use clap::Parser;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    date: String,
    category: String,
    is_done: bool,
    task: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Add { task, category }) => {
            let json = r#"
                {
                    "date": "2022",
                    "is_done": true,
                    "category": "dev",
                    "task": "some dev thing."
                }
                "#;

            let task: Task = serde_json::from_str(json).unwrap();

            println!("date: {}", task.date);
            println!("category: {}", task.category);
            println!("task: {}", task.task);
            println!("is done: {}", task.is_done);

        }
        Some(Commands::Delete { task_id }) => {}
        Some(Commands::Edit { task_id }) => {}
        Some(Commands::List { category }) => {}
        Some(Commands::Done { task_id }) => {}
        Some(Commands::Undone { task_id }) => {}
        Some(Commands::Clear { category }) => {}

        None => {}
    }
}
