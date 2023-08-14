// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod schema;

use commands::*;
use db::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

fn main() {
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tasks::get_tasks,
            get_categories::get_categories,
            add_task::add_task,
            add_category::add_category,
            remove_task::remove_task,
            update_task_status::update_task_status,
            update_task_title::update_task_title,
            update_task_category::update_task_category,
            remove_category::remove_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
