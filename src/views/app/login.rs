use super::content_loader::read_file;
use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    let mut html_data = read_file("./templates/login.html");
    let javascript_data: String = read_file("./javascript/login.js");
    let css_data: String = read_file("./css/main.css");
    let base_css_data: String = read_file("./css/base.css");

    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    html_data = html_data.replace("{{CSS}}", &css_data);
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
