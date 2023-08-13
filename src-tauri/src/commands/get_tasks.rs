#[derive(serde::Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub other_val: usize,
}

#[tauri::command]
pub fn get_tasks() -> Result<CustomResponse, String> {
    Ok(CustomResponse {
        message: "hello from rust".into(),
        other_val: 2,
    })
}
