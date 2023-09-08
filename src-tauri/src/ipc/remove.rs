use crate::db::establish_connection;
use crate::models::subtasks::SubTask;
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

#[tauri::command]
pub fn remove_task(id: String) -> Result<usize, String> {
    let mut connection = establish_connection();

    let result = match diesel::delete(schema::tasks::dsl::tasks.filter(schema::tasks::id.eq(&id)))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    };

    _ = diesel::delete(schema::subtasks::table.filter(schema::subtasks::id.eq(&id)))
        .execute(&mut connection);

    // Cleanup subtasks
    let q: Result<Vec<SubTask>, _> = schema::subtasks::table
        .select(SubTask::as_select())
        .filter(schema::subtasks::parent_id.eq(&id))
        .load(&mut connection);

    if let Ok(subtasks) = q {
        for sub_task in subtasks {
            _ = diesel::delete(schema::tasks::table.filter(schema::tasks::id.eq(sub_task.id)))
                .execute(&mut connection);
        }
    }

    _ = diesel::delete(schema::subtasks::table.filter(schema::subtasks::parent_id.eq(&id)))
        .execute(&mut connection);

    result
}
