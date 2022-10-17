// use this https://docs.rs/sqlite/0.27.0/sqlite/
// https://github.com/rusqlite/rusqlite

mod cli;
mod filesystem;
use clap::Parser;
use cli::{Cli, Commands};
use rusqlite::{Connection, Result};
use nanoid::nanoid;
use dotenv::dotenv;

#[derive(Debug)]
struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub is_done: bool
}

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    let db_file = match cli.file {
        Some(file) => file,
        None => String::from("todo.db")
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
                is_done: false
            };

            conn.execute(
                "INSERT INTO tasks (id, category, text, is_done) VALUES (?1, ?2, ?3, ?4)",
                (&new_task.id, &new_task.category, &new_task.text, &new_task.is_done),
            )?;
        }

        Some(Commands::Delete { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::Edit { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::List { category }) => {
            let mut stmt = conn.prepare("SELECT * FROM tasks")?;
            let tasks_iter = stmt.query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    is_done: row.get(3)?
                })
            })?;

            for task in tasks_iter {
                println!("Found task {:?}", task.unwrap());
            }
        }

        Some(Commands::Done { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::Undone { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::Clear { category }) => {
            println!("{:?}", category)
        }

        None => {}
    }

    Ok(())
}
