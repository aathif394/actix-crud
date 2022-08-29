use actix_web::{web, middleware, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;

mod handlers;
mod models;
mod schema;
mod ops;

use handlers::user;

#[macro_use]
extern crate diesel;
extern crate env_logger;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(user::index)
            .service(user::select)
            .service(user::create)
            .service(user::update)
            .service(user::delete)        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}