use super::content_loader::read_file;
use actix_web::HttpResponse;

/// Renders the main view that shows all items in the state.
///
/// # Returns
/// * (HttpResponse) with HTML
pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let javascript_data: String = read_file("./javascript/main.js");
    // Removed CSS data loading and replacement as it's now linked directly in HTML

    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    // Removed add_component call for header as it's now dynamically loaded by JavaScript

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
