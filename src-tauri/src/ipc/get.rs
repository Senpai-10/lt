use crate::db::establish_connection;
use crate::models::categories::Category;
use crate::models::tasks::Task;
use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RCategory {
    pub name: String,
    pub total_tasks_done: i64,
    pub total_tasks: i64,
}

#[derive(Deserialize, Serialize)]
pub struct CategoriesData {
    pub categories: Vec<RCategory>,
    pub total_tasks_done: i64,
    pub total_tasks: i64,
}

#[tauri::command]
pub fn get_categories() -> CategoriesData {
    let mut connection = establish_connection();
    let total_tasks: i64 = schema::tasks::dsl::tasks
        .count()
        .get_result::<i64>(&mut connection)
        .unwrap_or(0);

    let total_tasks_done: i64 = schema::tasks::dsl::tasks
        .filter(schema::tasks::status.eq(1))
        .count()
        .get_result::<i64>(&mut connection)
        .unwrap_or(0);

    let mut categories_data = CategoriesData {
        categories: Vec::new(),
        total_tasks_done,
        total_tasks,
    };

    let categories: Vec<Category> =
        match schema::categories::dsl::categories.load::<Category>(&mut connection) {
            Ok(r) => r,
            Err(_) => return categories_data,
        };

    for category in categories {
        let tasks_count: i64 = schema::tasks::dsl::tasks
            .filter(schema::tasks::category_name.eq(&category.name))
            .count()
            .get_result::<i64>(&mut connection)
            .unwrap_or(0);

        let done_count: i64 = schema::tasks::dsl::tasks
            .filter(schema::tasks::category_name.eq(&category.name))
            .filter(schema::tasks::status.eq(1))
            .count()
            .get_result::<i64>(&mut connection)
            .unwrap_or(0);

        let new_category = RCategory {
            name: category.name,
            total_tasks_done: done_count,
            total_tasks: tasks_count,
        };

        categories_data.categories.push(new_category);
    }

    categories_data
}

#[tauri::command]
pub fn get_tasks(category: Option<String>) -> Result<Vec<Task>, String> {
    let mut connection = establish_connection();

    match category {
        Some(name) => {
            match schema::tasks::dsl::tasks
                .filter(schema::tasks::category_name.eq(&name))
                .order(schema::tasks::priority.desc())
                .load(&mut connection)
            {
                Ok(r) => Ok(r),
                Err(e) => Err(e.to_string()),
            }
        }
        None => match schema::tasks::dsl::tasks
            .order(schema::tasks::priority.desc())
            .load(&mut connection)
        {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        },
    }
}
