use crate::db::establish_connection;
use crate::schema;
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
pub fn update_task_category(id: String, new_category: String) -> Result<usize, String> {
    let mut connection = establish_connection();

    let task = schema::tasks::dsl::tasks.filter(schema::tasks::id.eq(id));

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32;

    let _ = match diesel::update(task.clone())
        .set(schema::tasks::modification_date.eq(since_the_epoch))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    };

    match diesel::update(task)
        .set(schema::tasks::category_name.eq(new_category))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
