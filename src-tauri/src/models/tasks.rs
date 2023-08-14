// use crate::models::categories::Category;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub id: String,
    pub category_name: String,
    pub title: String,
    // status 0: todo, 1: INPROGRESS, 2: DONE
    pub status: i32,
    pub priority: i32,
    pub creation_date: i32,
    pub completion_date: Option<i32>,
    pub modification_date: i32,
}

#[derive(
    Queryable, AsChangeset, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(table_name = tasks)]
#[diesel(belongs_to(Category, foreign_key = category_name))]
pub struct Task {
    pub id: String,
    pub category_name: String,
    pub title: String,
    // status 0: todo, 1: INPROGRESS, 2: DONE
    pub status: i32,
    pub priority: i32,
    pub creation_date: i32,
    pub completion_date: Option<i32>,
    pub modification_date: i32,
}
