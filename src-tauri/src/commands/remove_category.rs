use crate::db::establish_connection;
use crate::schema;
use diesel::prelude::*;

#[tauri::command]
pub fn remove_category(name: String) -> Result<usize, String> {
    let mut connection = establish_connection();

    let _ = match diesel::delete(
        schema::tasks::dsl::tasks.filter(schema::tasks::category_name.eq(&name)),
    )
    .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    };

    match diesel::delete(
        schema::categories::dsl::categories.filter(schema::categories::name.eq(name)),
    )
    .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
