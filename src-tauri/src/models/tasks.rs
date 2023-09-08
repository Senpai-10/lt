use crate::models::categories::Category;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub id: String,
    pub category_name: String,
    pub title: String,
    pub desc: Option<String>,
    pub status: i32,
    pub priority: i32,
    pub is_child_task: i32,
    pub done_at: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(
    Identifiable,
    Associations,
    Queryable,
    AsChangeset,
    Selectable,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
)]
#[diesel(table_name = tasks)]
#[diesel(belongs_to(Category, foreign_key = category_name))]
#[diesel(treat_none_as_null = true)]
pub struct Task {
    pub id: String,
    pub category_name: String,
    pub title: String,
    pub desc: Option<String>,
    pub status: i32,
    pub priority: i32,
    pub is_child_task: i32,
    pub done_at: Option<i32>,
    pub created_at: i32,
    pub updated_at: i32,
}
