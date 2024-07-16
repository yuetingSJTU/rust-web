use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler( app_state: web::Data<AppState> ) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use super::models::Hookup;
use chrono::Utc;

pub async fn new_hookup(
    new_hookup: web::Json<Hookup>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    println!("Received new course");
    let hookup_count = app_state
        .hookups
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|hookup|hookup.friend_id == new_hookup.friend_id)
        .collect::<Vec<Hookup>>()
        .len();
    let new_hookup = Hookup {
        friend_id: new_hookup.friend_id,
        id: Some(hookup_count + 1),
        name: new_hookup.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.hookups.lock().unwrap().push(new_hookup);
    HttpResponse::Ok().json("Hookup added")
}