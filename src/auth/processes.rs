use actix_web::{FromRequest, HttpRequest, HttpResponse, Responder, web, Error};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;
use futures_util::future::{ready, Ready};
use log::{info, warn, error};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use jsonwebtoken::jwk::JwkSet; // Correctly import JwkSet from jsonwebtoken
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
    pub azp: String,
    pub preferred_username: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let claims = req.extensions().get::<Claims>().cloned();
        ready(match claims {
            Some(c) => Ok(c),
            None => {
                warn!("Claims not found in request extensions during FromRequest extraction. Returning 401 Unauthorized.");
                Err(ErrorUnauthorized("Unauthorized: Missing user claims"))
            }
        })
    }
}

/// Checks to see if the token matches and is valid using dynamic JWKS.
///
/// # Parameters
/// * token_string (String): The JWT to be validated.
/// * jwks_uri (&str): The URI to fetch the JSON Web Key Set from.
///
/// # Returns
/// * (Result<Claims, String>): Claims if the token is valid, an error message if not.
pub async fn check_password(token_string: String, jwks_uri: &str) -> Result<Claims, String> {
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

    validation.required_spec_claims.retain(|claim| claim != "exp"); // Remove "exp" from required claims if not needed
    validation.validate_aud = false; // Temporarily disable audience validation for debugging
    validation.set_issuer(&["http://localhost:8080/realms/myrealm"]); // Optional: Validate issuer

    match decode::<Claims>(&token_string, &decoding_key, &validation) {
        Ok(token_data) => {
            info!("Token validation successful. Claims: {:?}", token_data.claims);
            Ok(token_data.claims) // Return the Claims struct
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
pub fn extract_header_token(request: &HttpRequest) -> Result<String, &'static str> {
    info!("Attempting to extract Authorization header token.");
    match request.headers().get("Authorization") {
        Some(token) => {
            info!("'Authorization' header found.");
            match token.to_str() {
                Ok(token_str) => {
                    if token_str.starts_with("Bearer ") {
                        info!("Bearer token successfully extracted.");
                        Ok(token_str["Bearer ".len()..].to_string())
                    } else {
                        warn!("Authorization header does not start with 'Bearer '.");
                        Err("Invalid Authorization header format")
                    }
                },
                Err(_) => {
                    error!("Failed to process token from header (to_str conversion).");
                    Err("Error processing token")
                },
            }
        },
        None => {
            warn!("'Authorization' header not found in request.");
            Err("No Authorization header")
        },
    }
}
