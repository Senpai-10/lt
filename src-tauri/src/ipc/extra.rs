#[tauri::command]
pub fn quit_app() {
    std::process::exit(0);
}
