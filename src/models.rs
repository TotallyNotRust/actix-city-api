use diesel::{
    deserialize, insert_into,
    internal::derives,
    sql_types::{Integer, Nullable},
    Associations, Identifiable, Insertable, Queryable,
};
use serde::Deserialize;

use crate::schema::*;

#[derive(Queryable, Debug, PartialEq, Clone, Deserialize)]
#[diesel(table_name = City)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub country_id: i32,
}

#[derive(Queryable, Debug, PartialEq, Clone, Deserialize)]
#[diesel(table_name = Country)]
pub struct Country {
    pub id: i32,
    pub name: String,
}

