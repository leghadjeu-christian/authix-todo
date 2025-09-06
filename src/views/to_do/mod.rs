use actix_web::{web};

mod utils;
mod create;
mod get;
mod edit;
mod delete;
use super::path::Path;

/// This function adds the to-do item views to the web server.
///
/// # Arguments
/// * app: &mut web::ServiceConfig - the app service config
pub fn item_factory(app: &mut web::ServiceConfig) {
    // define the path struct
    let base_path: Path = Path { prefix: String::from("/item"), backend: true };

    // apply auth middleware only to this scoped group
    app.service(
        web::scope("/api/v1")
            .route(&base_path.define(String::from("/create/{title}")), web::post().to(create::create))
            .route(&base_path.define(String::from("/get")), web::get().to(get::get))
            .route(&base_path.define(String::from("/edit")), web::put().to(edit::edit))
            .route(&base_path.define(String::from("/delete")), web::post().to(delete::delete))
    );
}
