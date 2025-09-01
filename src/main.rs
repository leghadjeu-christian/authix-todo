#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
mod processes;
mod state;
mod to_do;
mod views;
mod json_serialization;
mod schema;
mod models;
mod database;
mod auth;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
