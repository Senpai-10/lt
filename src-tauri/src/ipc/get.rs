use crate::models::categories::Category;
use crate::models::tasks::Task;
use crate::schema;
use crate::{db::establish_connection, models::subtasks::SubTask};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskInRes {
    #[serde(flatten)]
    pub task: Task,
    pub sub_tasks: Vec<TaskInRes>,
}

fn get_sub_tasks(conn: &mut SqliteConnection, parent_task: &Task) -> Vec<TaskInRes> {
    schema::subtasks::table
        .filter(schema::subtasks::parent_id.eq(&parent_task.id))
        .load::<SubTask>(conn)
        .unwrap()
        .into_iter()
        .map(|subtask| {
            let task = schema::tasks::table
                .filter(schema::tasks::id.eq(subtask.id))
                .get_result::<Task>(conn)
                .unwrap();

            TaskInRes {
                sub_tasks: get_sub_tasks(conn, &task),
                task,
            }
        })
        .collect::<Vec<TaskInRes>>()
}

#[tauri::command]
pub fn get_tasks(category: Option<String>) -> Result<Vec<TaskInRes>, String> {
    let mut connection = establish_connection();

    let mut query = schema::tasks::table.into_boxed();

    if let Some(category_name) = category {
        query = query.filter(schema::tasks::category_name.eq(category_name))
    }

    match query
        .filter(schema::tasks::is_child_task.eq(0))
        .order(schema::tasks::priority.desc())
        .load::<Task>(&mut connection)
    {
        Ok(r) => Ok(r
            .into_iter()
            .map(|task| TaskInRes {
                sub_tasks: get_sub_tasks(&mut connection, &task),
                task,
            })
            .collect::<Vec<TaskInRes>>()),
        Err(e) => Err(e.to_string()),
    }
}
