use actix_web::web;
mod app;
mod auth;
mod path;
mod to_do;
pub mod users;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
    app::app_factory(app);
    users::user_factory(app);
}
