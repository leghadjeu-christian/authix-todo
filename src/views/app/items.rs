use super::content_loader::read_file;
use actix_web::{web, HttpResponse}; // Import web and HttpResponse
use crate::auth::KeycloakClientConfig; // Import KeycloakClientConfig

/// Renders the main view that shows all items in the state.
///
/// # Arguments
/// * (web::Data<KeycloakClientConfig>) Keycloak client configuration
///
/// # Returns
/// * (HttpResponse) with HTML
pub async fn items(keycloak_client_config: web::Data<KeycloakClientConfig>) -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let javascript_data_from_file: String = read_file("./javascript/main.js");

    let injected_javascript = format!(
        r#"
        window.KEYCLOAK_FRONTEND_URL = "{}";
        window.KEYCLOAK_FRONTEND_REALM = "{}";
        window.KEYCLOAK_FRONTEND_CLIENT_ID = "{}";
        "#,
        keycloak_client_config.auth_server_url,
        keycloak_client_config.realm,
        keycloak_client_config.client_id
    );

    let final_javascript = injected_javascript + &javascript_data_from_file;

    html_data = html_data.replace("{{JAVASCRIPT}}", &final_javascript);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
