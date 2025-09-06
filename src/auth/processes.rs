use actix_web::dev::ServiceRequest;
use log::{info, warn, error};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use jsonwebtoken::jwk::JwkSet; // Correctly import JwkSet from jsonwebtoken
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    // Define your JWT claims here. For Keycloak, typical claims include:
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
    // Add other claims as needed, e.g., preferred_username, realm_access, etc.
}

/// Checks to see if the token matches and is valid using dynamic JWKS.
///
/// # Parameters
/// * token_string (String): The JWT to be validated.
/// * jwks_uri (&str): The URI to fetch the JSON Web Key Set from.
///
/// # Returns
/// * (Result<String, String>): "passed" if the token is valid, an error message if not.
pub async fn check_password(token_string: String, jwks_uri: &str) -> Result<String, String> {
    info!("Attempting to check password/validate token using JWKS from: {}", jwks_uri);

    // 1. Decode the header to get the `kid` (Key ID)
    let header = match decode_header(&token_string) {
        Ok(h) => h,
        Err(e) => {
            warn!("Failed to decode JWT header: {}", e);
            return Err(format!("Invalid JWT header: {}", e));
        }
    };

    let kid = match header.kid {
        Some(k) => k,
        None => {
            warn!("JWT header does not contain 'kid'. Cannot identify key for validation.");
            return Err("JWT header missing 'kid'".to_string());
        }
    };
    info!("Extracted 'kid' from JWT header: {}", kid);

    // 2. Fetch the JWKS
    let jwks_response = match reqwest::get(jwks_uri).await {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to fetch JWKS from {}: {}", jwks_uri, e);
            return Err(format!("Failed to fetch JWKS: {}", e));
        }
    };

    let jwks_json: JwkSet = match jwks_response.json().await {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to parse JWKS JSON from {}: {}", jwks_uri, e);
            return Err(format!("Failed to parse JWKS: {}", e));
        }
    };
    info!("Successfully fetched and parsed JWKS.");

    // 3. Find the correct JWK using the `kid`
    let jwk = match jwks_json.keys.iter().find(|key| key.common.key_id.as_ref() == Some(&kid)) {
        Some(key) => key,
        None => {
            warn!("No JWK found with kid '{}' in the JWKS.", kid);
            return Err(format!("No matching JWK found for kid: {}", kid));
        }
    };
    info!("Found matching JWK for kid: {}", kid);

    // 4. Create a DecodingKey from the JWK
    let decoding_key = match DecodingKey::from_jwk(jwk) {
        Ok(key) => key,
        Err(e) => {
            error!("Failed to create DecodingKey from JWK: {}", e);
            return Err(format!("Failed to create decoding key: {}", e));
        }
    };
    info!("Successfully created DecodingKey from JWK.");

    // 5. Validate the token
    let mut validation = Validation::new(Algorithm::RS256); // Keycloak typically uses RS256
    // You might need to set `validation.validate_aud = false;` or specify expected audiences if `aud` claim is not a single string or if there are multiple valid audiences.
    // validation.set_issuer(&["http://localhost:8080/realms/myrealm"]); // Optional: Validate issuer

    match decode::<Claims>(&token_string, &decoding_key, &validation) {
        Ok(token_data) => {
            info!("Token validation successful. Claims: {:?}", token_data.claims);
            Ok("passed".to_string())
        },
        Err(e) => {
            warn!("Token validation failed: {}", e);
            Err(format!("Token validation failed: {}", e))
        },
    }
}

/// Extracts the header from the request.
///
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function
///
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, error message if not
pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    info!("Attempting to extract header token.");
    match request.headers().get("user-token") {
        Some(token) => {
            info!("'user-token' header found.");
            match token.to_str() {
                Ok(processed_password) => {
                    info!("Token successfully processed from header.");
                    Ok(String::from(processed_password))
                },
                Err(_processed_password) => {
                    error!("Failed to process token from header (to_str conversion).");
                    Err("there was an error processing token")
                },
            }
        },
        None => {
            warn!("'user-token' header not found in request.");
            Err("there is no token")
        },
    }
}
