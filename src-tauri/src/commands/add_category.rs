use crate::db::establish_connection;
use crate::models::categories::NewCategory;
use crate::schema;
use diesel::prelude::*;

#[tauri::command]
pub fn add_category(name: String) -> Result<usize, String> {
    let mut connection = establish_connection();

    let new_category = NewCategory { name };

    match diesel::insert_into(schema::categories::dsl::categories)
        .values(new_category)
        .execute(&mut connection)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
