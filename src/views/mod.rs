use actix_web::web;
mod app;
mod auth;
mod path;
mod to_do;
pub mod users;

use actix_web_middleware_keycloak_auth::{KeycloakAuth, AlwaysReturnPolicy};

pub fn views_factory(app: &mut web::ServiceConfig, keycloak_auth: KeycloakAuth<AlwaysReturnPolicy>) {
    auth::auth_factory(app);
    to_do::item_factory(app, keycloak_auth);
    app::app_factory(app);
    users::user_factory(app);
}
