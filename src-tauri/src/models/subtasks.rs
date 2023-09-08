use crate::schema::subtasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = subtasks)]
pub struct NewSubTask {
    pub id: String,
    pub parent_id: String,
}

#[derive(
    Identifiable, Queryable, AsChangeset, Selectable, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(table_name = subtasks)]
#[diesel(belongs_to(Task, foreign_key = id))]
#[diesel(belongs_to(Task, foreign_key = child_id))]
pub struct SubTask {
    pub id: String,
    pub parent_id: String,
}
