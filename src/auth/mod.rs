use actix_web::dev::ServiceRequest;
use actix_web::web; // Add this for web::Data
// pub mod jwt; // No longer needed
mod processes;
pub mod keycloak_config; // Added for Keycloak OpenID Connect configuration fetching
use log::{info, warn, error}; // Added error for consistency

pub async fn process_token(request: &ServiceRequest, jwks_uri: web::Data<String>) -> Result<String, String> {
    info!("Attempting to process token in auth::mod.rs");
    let jwks_uri_str: &str = &jwks_uri; // Dereference web::Data to &String, then to &str

    match processes::extract_header_token(request) {
        Ok(token) => {
            info!("Header token extracted successfully.");
            match processes::check_password(token, jwks_uri_str).await { // Pass jwks_uri and await
                Ok(_) => {
                    info!("Token validation successful.");
                    Ok(String::from("passed"))
                },
                Err(message) => {
                    warn!("Token validation failed: {}", message);
                    Err(message)
                }
            }
        },
        Err(message) => {
            warn!("Token extraction failed: {}", message);
            Err(message.to_string()) // Convert &'static str to String
        }
    }
}
