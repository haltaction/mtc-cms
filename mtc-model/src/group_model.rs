use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GroupModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub title: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Deserialize, Validate)]
pub struct GroupCreateModel {
    pub title: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct GroupUpdateModel {
    pub title: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct GroupsModel {
    pub groups: Vec<String>,
}