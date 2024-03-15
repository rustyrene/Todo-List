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
        .route(
            "/rename-category/{category_id}",
            web::patch().to(rename_category),
        )
        .route(
            "/get-all-user-categories",
            web::get().to(get_category_from_user),
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

#[derive(Serialize, Deserialize)]
struct CategoryRows {
    categories: Vec<Category>,
}

async fn get_category_from_user(
    auth_token: AuthenticationToken,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let user_id = auth_token.user_id;

    let categories = sqlx::query!(
        "SELECT * FROM category_table WHERE category_id = $1",
        user_id,
    )
    .fetch_all(&app_state.pool)
    .await;

    match categories {
        Ok(categories) => {
            let category_records = categories
                .into_iter()
                .map(|row| Category {
                    category_id: row.category_id,
                    category_name: row.category_name,
                    user_id: row.user_id.unwrap(),
                })
                .collect();

            return HttpResponse::Ok().json(CategoryRows {
                categories: category_records,
            });
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(Response {
                message: e.to_string(),
            })
        }
    }
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

async fn rename_category(
    path: web::Path<Uuid>,
    auth_token: AuthenticationToken,
    app_state: web::Data<AppState>,
    body: web::Json<CategoryJson>,
) -> HttpResponse {
    let category_id: Uuid = path.into_inner();
    let user_id: Uuid = auth_token.user_id;
    let category_name: String = body.category_name.clone();

    let query = sqlx::query_as!(
        Category,
        "UPDATE category_table SET category_name = $1 WHERE category_id = $2 AND user_id = $3",
        category_name,
        category_id,
        user_id,
    )
    .fetch_optional(&app_state.pool)
    .await;

    match query {
        Ok(_) => {
            return HttpResponse::Ok().json(Response {
                message: "Success".to_string(),
            })
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(Response {
                message: e.to_string(),
            })
        }
    }
}
