use actix_web::dev::ServiceRequest;
use actix_web::web;
pub mod processes;
pub mod keycloak_config;
use log::{info, warn};
use crate::auth::processes::Claims; // Import Claims struct

pub async fn process_token(request: &ServiceRequest, jwks_uri: web::Data<String>) -> Result<Claims, String> {
    info!("Attempting to process token in auth::mod.rs");
    let jwks_uri_str: &str = &jwks_uri;

    match processes::extract_header_token(request) {
        Ok(token) => {
            info!("Header token extracted successfully.");
            match processes::check_password(token, jwks_uri_str).await {
                Ok(claims) => {
                    info!("Token validation successful. User: {}", claims.sub);
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
