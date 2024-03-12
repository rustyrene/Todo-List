use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{extractors::authentication_token::AuthenticationToken, AppState, Response};

pub fn category_scope() -> Scope {
    web::scope("/category")
        .route("/add-category", web::post().to(add_category))
        .route(
            "/delete-category/{catecory_id}",
            web::delete().to(delete_category),
        )
}

#[derive(Serialize, Deserialize)]
struct Category {
    category_id: Uuid,
    category_name: String,
    user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct CategoryJson {
    category_name: String,
}

async fn add_category(
    auth_token: AuthenticationToken,
    body: web::Json<CategoryJson>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let user_id = auth_token.user_id;
    let category_name = body.category_name.clone();

    match sqlx::query_as!(
        Category,
        "INSERT INTO category_table (category_id, category_name, user_id) VALUES ($1, $2, $3)",
        Uuid::new_v4(),
        category_name,
        user_id
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(_) => {
            return HttpResponse::Ok().json(Response {
                message: "Category added successfully".to_string(),
            })
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(Response {
                message: e.to_string(),
            })
        }
    }
}

async fn delete_category(
    auth_token: AuthenticationToken,
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let category_id = path.into_inner();
    let user_id = auth_token.user_id;

    match sqlx::query!(
        "DELETE FROM category_table WHERE category_id = $1 AND user_id = $2",
        category_id,
        user_id,
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(_) => {
            return HttpResponse::Ok().json(Response {
                message: "Category deleted successfully".to_string(),
            })
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(Response {
                message: e.to_string(),
            });
        }
    }
}
