use crate::db::establish_connection;
use crate::models::categories::NewCategory;
use crate::models::subtasks::NewSubTask;
use crate::models::tasks::NewTask;
use crate::schema;
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::prelude::*;
use nanoid::nanoid;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[tauri::command]
pub fn add_task(title: String, category: String) -> Result<usize, String> {
    let mut connection = establish_connection();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32;

    let query = select(exists(
        schema::categories::dsl::categories.filter(schema::categories::name.eq(&category)),
    ))
    .get_result::<bool>(&mut connection);
    if let Ok(category_exists) = query {
        if !category_exists {
            let new_category = NewCategory {
                name: category.clone(),
            };

            _ = diesel::insert_into(schema::categories::dsl::categories)
                .values(new_category)
                .execute(&mut connection);
        }
    }

    let new_task = NewTask {
        id: nanoid!(),
        category_name: category,
        title,
        desc: None,
        status: 0,
        priority: 0,
        is_child_task: 0,
        created_at: since_the_epoch,
        done_at: None,
        updated_at: since_the_epoch,
    };

    match diesel::insert_into(schema::tasks::dsl::tasks)
        .values(new_task)
        .execute(&mut connection)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn add_subtask(parent_id: String, title: String, category: String) -> Result<usize, String> {
    let mut connection = establish_connection();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32;

    let query = select(exists(
        schema::categories::dsl::categories.filter(schema::categories::name.eq(&category)),
    ))
    .get_result::<bool>(&mut connection);
    if let Ok(category_exists) = query {
        if !category_exists {
            let new_category = NewCategory {
                name: category.clone(),
            };

            _ = diesel::insert_into(schema::categories::dsl::categories)
                .values(new_category)
                .execute(&mut connection);
        }
    }

    let new_task = NewTask {
        id: nanoid!(),
        category_name: category,
        title,
        desc: None,
        status: 0,
        priority: 0,
        is_child_task: 1,
        created_at: since_the_epoch,
        done_at: None,
        updated_at: since_the_epoch,
    };

    let result = match diesel::insert_into(schema::tasks::dsl::tasks)
        .values(&new_task)
        .execute(&mut connection)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    };

    let new_subtask = NewSubTask {
        id: new_task.id,
        parent_id,
    };

    _ = diesel::insert_into(schema::subtasks::table)
        .values(new_subtask)
        .execute(&mut connection);

    result
}
