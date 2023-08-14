use crate::db::establish_connection;
use crate::models::categories::Category;
use crate::schema;
use diesel::prelude::*;

#[tauri::command]
pub fn get_categories() -> Result<Vec<Category>, String> {
    let mut connection = establish_connection();

    match schema::categories::dsl::categories.load(&mut connection) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
