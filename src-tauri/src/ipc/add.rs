use crate::db::establish_connection;
use crate::models::categories::NewCategory;
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
        status: 0,
        priority: 0,
        creation_date: since_the_epoch,
        completion_date: None,
        modification_date: since_the_epoch,
    };

    match diesel::insert_into(schema::tasks::dsl::tasks)
        .values(new_task)
        .execute(&mut connection)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
