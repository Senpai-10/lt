use crate::schema::categories;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = categories)]
#[diesel(primary_key(name))]
pub struct Category {
    pub name: String,
}
