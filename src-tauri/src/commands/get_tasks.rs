use crate::db::establish_connection;
use crate::models::tasks::Task;
use crate::schema;
use diesel::prelude::*;

#[tauri::command]
pub fn get_tasks(category: Option<String>) -> Result<Vec<Task>, String> {
    let mut connection = establish_connection();

    match category {
        Some(name) => {
            match schema::tasks::dsl::tasks
                .filter(schema::tasks::category_name.eq(&name))
                .load(&mut connection)
            {
                Ok(r) => Ok(r),
                Err(e) => Err(e.to_string()),
            }
        }
        None => match schema::tasks::dsl::tasks.load(&mut connection) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        },
    }
}
