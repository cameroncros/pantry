use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{ToSchema};

use crate::schema::items;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, PartialEq, AsChangeset, ToSchema)]
#[schema(example = json!({"id": 1, "description": "Lasagna", "date": "2024-01-01"}))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub description: String,
    #[schema(value_type = Option<String>)]
    pub date: Option<NaiveDate>,
}
