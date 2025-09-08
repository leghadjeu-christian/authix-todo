use actix_web::{web, HttpRequest};
use actix_web::HttpMessage; // Import HttpMessage trait for extensions_mut()
use log::{info, warn, error};
pub mod processes; // Make processes module public
pub mod keycloak_config;
use crate::auth::processes::Claims;

#[derive(Clone, Debug)]
pub struct KeycloakClientConfig {
    pub auth_server_url: String,
    pub realm: String,
    pub client_id: String,
}

pub async fn process_token(request: &HttpRequest, jwks_uri: web::Data<String>) -> Result<Claims, String> {
    info!("Attempting to process token in auth::mod.rs");
    let jwks_uri_str: &str = &jwks_uri;

    match processes::extract_header_token(request) {
        Ok(token) => {
            info!("Authorization header token extracted successfully.");
            match processes::check_password(token, jwks_uri_str).await {
                Ok(claims) => {
                    info!("Token validation successful. User ID: {}", claims.sub);
                    // Insert claims into the request extensions for later use by route handlers
                    request.extensions_mut().insert(claims.clone());
                    Ok(claims)
                },
                Err(message) => {
                    warn!("Token validation failed: {}", message);
                    Err(message)
                }
            }
        },
        Err(message) => {
            warn!("Token extraction failed: {}", message);
            Err(message.to_string())
        }
    }
}
