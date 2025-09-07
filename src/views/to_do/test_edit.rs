use actix_web::{web, HttpResponse};
use log::{info, warn};
use crate::models::item::update_item::UpdateItem;
use crate::auth::processes::Claims; // Re-add Claims import

pub async fn test_edit_json(claims: Claims, update_data: web::Json<UpdateItem>) -> HttpResponse {
    warn!("TEST: Reached test_edit_json handler.");
    info!("TEST: Received update_data in test_edit_json: {:?}", update_data);
    HttpResponse::Ok().json("Test JSON received successfully!")
}