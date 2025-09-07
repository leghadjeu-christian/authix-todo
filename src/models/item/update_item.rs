use diesel::{AsChangeset, Insertable, Queryable};
use serde::Deserialize;

use crate::schema::to_do;

#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "to_do"]
pub struct UpdateItem {
    pub title: String,
    pub status: String,
}