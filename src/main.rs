use actix_web::{get, post, patch, delete, web, App, HttpServer, HttpResponse, Responder};
use crate::models::{User, NewUser, DelUser};

mod models;
mod ops;
mod schema;
mod db;

use ops::user_ops;

#[macro_use]
extern crate diesel;

#[get("/users")]
async fn get() -> impl Responder {

    let data = user_ops::get_users();
    HttpResponse::Ok().json(data)

}


#[post("/users")]
async fn create(user: web::Json<NewUser>) -> impl Responder{
    // format!("Hello {name}!")

    let new_user = NewUser {
        name: user.name.to_string()
    };

    user_ops::create_user(new_user);
    HttpResponse::Ok().json(user)

}


#[patch("/users")]
async fn update(user: web::Json<User>) -> impl Responder {
    let upd_user = User {
        id: user.id,
        name: user.name.to_string()
    };

    user_ops::update_user(upd_user);
    HttpResponse::Ok().json(user)

}

#[delete("/users")]
async fn delete(user: web::Json<DelUser>) -> impl Responder {
    let del_user = DelUser {
        id: user.id,
    };

    user_ops::delete_user(del_user);
    HttpResponse::Ok().json(user)

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(get)
            .service(create)
            .service(update)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}