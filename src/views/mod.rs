use actix_web::web;
mod app;
mod auth;
mod path;
mod to_do;
pub mod users;

// use actix_web_middleware_keycloak_auth::{KeycloakAuth, AlwaysReturnPolicy}; // Removed KeycloakAuth import

pub fn views_factory(app: &mut web::ServiceConfig) { // Removed keycloak_auth parameter
    auth::auth_factory(app);
    to_do::item_factory(app); // Removed keycloak_auth from call
    app::app_factory(app);
    users::user_factory(app);
}
