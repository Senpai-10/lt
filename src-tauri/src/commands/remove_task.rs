use crate::db::establish_connection;
use crate::schema;
use diesel::prelude::*;

#[tauri::command]
pub fn remove_task(id: String) -> Result<usize, String> {
    let mut connection = establish_connection();

    match diesel::delete(schema::tasks::dsl::tasks.filter(schema::tasks::id.eq(id)))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
