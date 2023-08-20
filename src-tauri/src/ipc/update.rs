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

#[tauri::command]
pub fn update_task_priority(id: String, new_priority: i32) -> Result<usize, String> {
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
        .set(schema::tasks::priority.eq(new_priority))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_task_status(id: String, status: i32) -> Result<usize, String> {
    let mut connection = establish_connection();

    let task = schema::tasks::dsl::tasks.filter(schema::tasks::id.eq(id));

    if status == 1 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i32;

        let _ = match diesel::update(task.clone())
            .set(schema::tasks::completion_date.eq(since_the_epoch))
            .execute(&mut connection)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };

        let _ = match diesel::update(task.clone())
            .set(schema::tasks::priority.eq(-1))
            .execute(&mut connection)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };

        let _ = match diesel::update(task.clone())
            .set(schema::tasks::modification_date.eq(since_the_epoch))
            .execute(&mut connection)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };
    } else if status == 0 {
        let _ = match diesel::update(task.clone())
            .set(schema::tasks::completion_date.eq(None::<i32>))
            .execute(&mut connection)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };

        let _ = match diesel::update(task.clone())
            .set(schema::tasks::priority.eq(0))
            .execute(&mut connection)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };
    }

    match diesel::update(task)
        .set(schema::tasks::status.eq(status))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_task_title(id: String, title: String) -> Result<usize, String> {
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
        .set(schema::tasks::title.eq(title))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn update_task_desc(id: String, desc: String) -> Result<usize, String> {
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
        .set(schema::tasks::desc.eq(desc))
        .execute(&mut connection)
    {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
