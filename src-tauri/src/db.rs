use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    #[cfg(debug_assertions)]
    let database_url = env::var("DATABASE_URL").unwrap_or(".todo-lt".into());

    #[cfg(not(debug_assertions))]
    let database_url = ".todo-lt";

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
