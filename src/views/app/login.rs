use super::content_loader::read_file;
use actix_web::{web, HttpResponse}; // Import web and HttpResponse
use crate::auth::KeycloakClientConfig; // Import KeycloakClientConfig

pub async fn login(keycloak_client_config: web::Data<KeycloakClientConfig>) -> HttpResponse {
    let mut html_data = read_file("./templates/login.html");
    let javascript_data_from_file: String = read_file("./javascript/login.js");

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

    // The login.html also pulls main.js, which needs these. Ensure they are available.
    html_data = html_data.replace("{{JAVASCRIPT}}", &final_javascript);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
