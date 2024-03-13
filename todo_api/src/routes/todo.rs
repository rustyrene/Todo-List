use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{extractors::authentication_token::AuthenticationToken, AppState, Response};

pub fn todo_scope() -> Scope {
    web::scope("/todo").route("/add-todo", web::post().to(add_todo))
}

#[derive(Serialize, Deserialize)]
struct Todo {
    todo_id: Uuid,
    todo_name: String,
    is_done: bool,
    user_id: Uuid,
    category_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct TodoJson {
    todo_name: String,
    is_done: Option<bool>,
    category_id: Uuid,
}

async fn add_todo(
    auth_token: AuthenticationToken,
    body: web::Json<TodoJson>,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let todo_name = body.todo_name.clone();
    let user_id = auth_token.user_id;
    let category_id = body.category_id;
    let is_done = match body.is_done {
        Some(is_done) => is_done,
        None => false,
    };
    let todo_id = path.into_inner();

    match sqlx::query_as!(Todo, "INSERT INTO todo_table (todo_id, todo_name, is_done, user_id, category_id) VALUES ($1, $2, $3, $4, $5)",
        todo_id,
        todo_name,
        is_done,
        user_id,
        category_id,
    ).execute(&app_state.pool).await {
            Ok(_) => return HttpResponse::Ok().json(Response{ message:"Todo added successfully".to_string() }),
            Err(e) => return HttpResponse::BadRequest().json(Response { message:e.to_string() }),
        }
}
