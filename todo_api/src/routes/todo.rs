use actix_web::{web, HttpResponse, Scope};

use crate::AppState;

pub fn todo_scope() -> Scope {
    web::scope("/todo").route("/create-category", web::post().to(create_category))
}

async fn create_category(app_state: web::Data<AppState>) -> HttpResponse {
    let pool = &app_state.pool;
    HttpResponse::Ok().finish()
}
