#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, web};
use log::{info, error}; // Added for logging
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};
use actix_files as fs; // Import actix_files

use env_logger;
mod auth;
use crate::auth::keycloak_config::fetch_keycloak_openid_config; // Import the function to fetch OIDC config
mod schema;
mod database;
mod processes;
mod models;
mod state;
mod to_do;
mod json_serialization;
mod views;
mod middleware; // Add this line to include the middleware module
use crate::middleware::request_logger::RequestLogger; // Import our custom RequestLogger middleware explicitly

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting Actix Web application...");

    const KEYCLOAK_BASE_URL: &str = "http://localhost:8080/realms/myrealm";
    let jwks_uri: String = match fetch_keycloak_openid_config(KEYCLOAK_BASE_URL).await {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to fetch JWKS URI from Keycloak: {}", e);
            // Handle the error appropriately, perhaps by exiting or using a default.
            // For now, we'll panic to clearly indicate a critical setup failure.
            panic!("Critical error: Could not obtain JWKS URI from Keycloak.")
        }
    };
    let jwks_uri_data = web::Data::new(jwks_uri); // Store JWKS URI in app state


    let keycloak_auth = KeycloakAuth::default_with_pk(
        DecodingKey::from_rsa_pem(include_bytes!("../keycloak_pub.pem")).unwrap()
    );

    HttpServer::new(move || {
        let keycloak_auth = keycloak_auth.clone();
        let jwks_uri_data = jwks_uri_data.clone(); // Clone for each worker
        info!("Setting up application routes and middleware.");
        let app = App::new()
            .app_data(jwks_uri_data.clone()) // Add JWKS URI to app data
            .service(fs::Files::new("/javascript", "./javascript").show_files_listing()) // Serve static files
            .service(fs::Files::new("/css", "./css").show_files_listing()) // Serve CSS files
            .service(fs::Files::new("/templates", "./templates").show_files_listing()) // Serve templates (including header.html)
            .wrap(RequestLogger) // Use our custom RequestLogger middleware

            .configure(move |cfg| {
                views::views_factory(cfg, keycloak_auth.clone())
            });
        return app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
