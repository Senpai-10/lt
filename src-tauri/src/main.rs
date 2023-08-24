// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod ipc;
mod models;
mod schema;

use db::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use ipc::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

fn main() {
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            get_categories,
            add_task,
            add_category,
            remove_task,
            update_task_status,
            update_task_title,
            update_task_category,
            remove_category,
            update_task_priority,
            update_task_desc,
            quit_app,
            get_current_theme,
            get_theme_css,
            set_theme,
            get_themes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
