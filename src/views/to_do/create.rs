use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use diesel::prelude::*;
use crate::auth::processes::Claims; // Import Claims
use actix_web::HttpMessage; // To access request extensions

use super::utils::return_state;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().expect("Claims not found in request extensions");
    let user_id = claims.sub.clone();
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_ref: String = title.clone();
    let mut connection = establish_connection();
    let items = to_do::table
    .filter(to_do::columns::title.eq(title_ref.as_str()))
    .filter(to_do::columns::user_id.eq(&user_id))
    .order(to_do::columns::id.asc())
    .select(to_do::all_columns)
    .load::<Item>(&mut connection)
    .unwrap();
    
    if items.len() == 0 {
        let new_post = NewItem::new(title, user_id.clone());
        let _ = diesel::insert_into(to_do::table)
        .values(&new_post)
        .execute(&mut connection);
    }
    HttpResponse::Ok().json(return_state(&user_id))
}