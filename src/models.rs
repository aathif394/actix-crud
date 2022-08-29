use crate::schema::users;
use serde::{Serialize, Deserialize};
use diesel::*;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String
}

#[derive(Debug, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct DelUser {
    pub id: i32
}