use crate::db::establish_connection;
use crate::models::{User, NewUser, DelUser};
use diesel::prelude::*;


pub fn get_users() -> Vec<User> {
    println!("Getting users");
    use crate::schema::users::dsl::*;

    let conn = establish_connection();

    let results: Vec<User> = users
        .load::<User>(&conn)
        .unwrap();

    results
}

pub fn create_user(user: NewUser) {
    println!("Creating user: {:#?}", user);
    use crate::schema::users::dsl::*;

    let conn = establish_connection();

    diesel::insert_into(users)
        .values(&user)
        .execute(&conn)
        .expect("Error creating new user");
}

pub fn update_user(user: User) {
    println!("Updating user: {:#?}", user);
    use crate::schema::users::dsl::*;


    let conn = establish_connection();
    
    diesel::update(users.find(user.id))
        .set(name.eq(user.name))
        .execute(&conn);
}

pub fn delete_user(user: DelUser) {
    println!("Deleting user: {:#?}", user);

    use crate::schema::users::dsl::*;


    let conn = establish_connection();

    diesel::delete(users.find(user.id))
    .execute(&conn);
}