#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, web, HttpResponse, Error};
use actix_service::Service;
use futures::future::{ok, Either, Ready};
use log::{info, warn, error}; // Added for logging
use actix_files as fs; // Import actix_files

use env_logger;
mod auth;
use crate::auth::keycloak_config::fetch_keycloak_openid_config; // Import the function to fetch OIDC config
use crate::auth::KeycloakClientConfig; // Import the new struct
mod schema;
mod database;
mod processes;
mod models;
mod state;
mod to_do;
mod json_serialization;
mod views;
mod middleware; 
use crate::middleware::request_logger::RequestLogger; // Import our custom RequestLogger middleware explicitly

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting Actix Web application...");

    // Load Keycloak configuration from environment variables with default values
    let keycloak_auth_server_url = std::env::var("KEYCLOAK_AUTH_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080/".to_string());
    let keycloak_realm = std::env::var("KEYCLOAK_REALM")
        .unwrap_or_else(|_| "myrealm".to_string());
    let keycloak_client_id = std::env::var("KEYCLOAK_CLIENT_ID")
        .unwrap_or_else(|_| "myclient".to_string());

    info!("Using Keycloak Auth Server URL: {}", keycloak_auth_server_url);
    info!("Using Keycloak Realm: {}", keycloak_realm);
    info!("Using Keycloak Client ID: {}", keycloak_client_id);

    // Construct the Keycloak base URL for OpenID Connect discovery
    let keycloak_openid_base_url = format!("{}/realms/{}", keycloak_auth_server_url.trim_end_matches('/'), keycloak_realm);
    info!("Constructed Keycloak OpenID Base URL: {}", keycloak_openid_base_url);

    let jwks_uri: String = match fetch_keycloak_openid_config(&keycloak_openid_base_url).await {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to fetch JWKS URI from Keycloak: {}", e);
            panic!("Critical error: Could not obtain JWKS URI from Keycloak.")
        }
    };
    let jwks_uri_data = web::Data::new(jwks_uri); // Store JWKS URI in app state

    // Create KeycloakClientConfig data for frontend and other parts of the backend
    let keycloak_client_config = web::Data::new(KeycloakClientConfig {
        auth_server_url: keycloak_auth_server_url.clone(),
        realm: keycloak_realm.clone(),
        client_id: keycloak_client_id.clone(),
    });

    HttpServer::new(move || {
        let jwks_uri_data = jwks_uri_data.clone(); // Clone for each worker
        let keycloak_client_config = keycloak_client_config.clone(); // Clone for each worker
        info!("Setting up application routes and middleware.");
        let app = App::new()
            .app_data(jwks_uri_data.clone()) // Add JWKS URI to app data
            .app_data(keycloak_client_config.clone()) // Add Keycloak client config to app data
            .service(fs::Files::new("/javascript", "./javascript").show_files_listing()) // Serve static files
            .service(fs::Files::new("/css", "./css").show_files_listing()) // Serve CSS files
            .service(fs::Files::new("/templates", "./templates").show_files_listing()) // Serve templates (including header.html)
            .wrap(RequestLogger) // Use our custom RequestLogger middleware

            .configure(move |cfg| {
                views::views_factory(cfg)
            });
        return app
    })
    .bind(std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string()))?
    .run()
    .await
}
