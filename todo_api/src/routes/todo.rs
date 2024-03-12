use actix_web::{web, HttpResponse, Scope};

use crate::{extractors::authentication_token::AuthenticationToken, Response};

pub fn todo_scope() -> Scope {
    web::scope("/todo").route("/add-todo", web::post().to(add_todo))
}

async fn add_todo(auth_token: AuthenticationToken) -> HttpResponse {
    HttpResponse::Ok().json({
        Response {
            message: format!("{}", auth_token.user_id),
        }
    })
}
