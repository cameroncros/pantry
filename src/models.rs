use serde::{Deserialize, Serialize};

use crate::schema::items;

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub description: String,
}
