// use this https://docs.rs/sqlite/0.27.0/sqlite/

mod cli;
mod filesystem;
use clap::Parser;
use cli::{Cli, Commands};
// use sqlx::{sqlite::SqliteConnection, sqlite::SqliteRow, Connection, Sqlite};
use dotenv::dotenv;

struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub is_done: bool
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let cli = Cli::parse();

    // let db_path: String = match std::env::var("DATABASE_URL") {
    //     Ok(v) => {
    //         v
    //     },
    //     Err(_) => {
    //         "test.db".into()
    //     }
    // };

    // if filesystem::file_exists(&db_path) {
    //     filesystem::create_file(&db_path, "")
    // }

    // let mut conn = SqliteConnection::connect(&db_path).await?;

    // sqlx::query(
    //     r#"
    //     CREATE TABLE IF NOT EXISTS tasks (
    //         id          VARCHAR     NOT NULL PRIMARY KEY,
    //         category    VARCHAR     NOT NULL,
    //         text        TEXT        NOT NULL,
    //         is_done     BOOLEAN     NOT NULL DEFAULT 'false'
    //     )
    // "#).execute(&mut conn).await?;

    // sqlx::query(
    //     r#"
    //     INSERT INTO tasks (id, text) VALUES (1, 'some testing text')
    // "#).execute(&mut conn).await?;

    // sqlx::query(
    //     r#"
    //     INSERT INTO tasks (id, text) VALUES (2, 'some testing text number 2')
    // "#).execute(&mut conn).await?;

    // sqlx::query(
    //     r#"
    //     INSERT INTO tasks (id, text) VALUES (3, 'some testing text number 3')
    // "#).execute(&mut conn).await?;

    match &cli.commands {
        Some(Commands::Add { task, category }) => {}

        Some(Commands::Delete { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::Edit { task_id }) => {
            println!("{}", task_id)
        }

        Some(Commands::List { category }) => {
            // let res: Vec<Task> = sqlx::query_as!(Task,
            //     r#"
            //     SELECT * FROM tasks
            //     "#
            // )
            // .fetch_all(&mut conn)
            // .await.unwrap();

            // for task in res {
            //     println!("id: {}", task.id);
            //     println!("text: {}", task.text);
            // }
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
