use crate::db::establish_connection;
use crate::models::categories::Category;
use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RCategory {
    pub name: String,
    pub total_tasks: i64,
}

#[derive(Deserialize, Serialize)]
pub struct CategoriesData {
    pub categories: Vec<RCategory>,
    pub total_tasks: i64,
}

#[tauri::command]
pub fn get_categories() -> CategoriesData {
    let mut connection = establish_connection();
    let all_tasks_count: i64 = schema::tasks::dsl::tasks
        .count()
        .get_result::<i64>(&mut connection)
        .unwrap_or(0);

    let mut categories_data = CategoriesData {
        categories: Vec::new(),
        total_tasks: all_tasks_count,
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

        let new_category = RCategory {
            name: category.name,
            total_tasks: tasks_count,
        };

        categories_data.categories.push(new_category);
    }

    categories_data
}
