use serde::{Serialize, Deserialize};
use crate::schema::users;
use chrono;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserPayload {
    pub name: String,
    pub email: String
}