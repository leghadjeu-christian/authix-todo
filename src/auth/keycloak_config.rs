use serde::Deserialize;
use reqwest;
use log::{info, warn, error};

#[derive(Debug, Deserialize)]
pub struct OpenIdConfig {
    pub jwks_uri: String,
    // Add other fields from the OpenID Connect configuration that might be useful
}

pub async fn fetch_keycloak_openid_config(keycloak_base_url: &str) -> Result<String, String> {
    let config_url = format!("{}/.well-known/openid-configuration", keycloak_base_url);
    info!("Attempting to fetch Keycloak OpenID Connect configuration from: {}", config_url);

    match reqwest::get(&config_url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<OpenIdConfig>().await {
                    Ok(config) => {
                        info!("Successfully fetched Keycloak OpenID Connect configuration.");
                        info!("JWKS URI: {}", config.jwks_uri);
                        Ok(config.jwks_uri)
                    },
                    Err(e) => {
                        error!("Failed to parse OpenID Connect configuration JSON: {}", e);
                        Err(format!("Failed to parse OpenID Connect configuration: {}", e))
                    },
                }
            } else {
                let status = response.status();
                let text = response.text().await.unwrap_or_else(|_| "N/A".to_string());
                error!("Failed to fetch OpenID Connect configuration: HTTP Status {}, Body: {}", status, text);
                Err(format!("Failed to fetch OpenID Connect configuration: HTTP Status {}", status))
            }
        },
        Err(e) => {
            error!("Failed to make HTTP request to fetch OpenID Connect configuration: {}", e);
            Err(format!("Failed to make HTTP request: {}", e))
        },
    }
}