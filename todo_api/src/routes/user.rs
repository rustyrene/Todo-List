use actix_web::{web, HttpResponse, Scope};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::AppState;

pub fn user_scope() -> Scope {
    web::scope("/user").route("/sign-up", web::post().to(user_signup))
}

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

/**
This function returns Ok(true) if the user with the given username already exists. Ok(false) if username doesnt exist already.

# Argguments

* 'pool' - A Postgres connection pool to the database
* 'username' - The username that is searched for in the database

*/
async fn user_exists(pool: &PgPool, username: String) -> Result<bool, sqlx::Error> {
    let user = sqlx::query!("SELECT * FROM user_table WHERE user_name = $1", username)
        .fetch_optional(pool)
        .await?;

    match user {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

async fn user_signup(body: web::Json<Login>, app_state: web::Data<AppState>) -> HttpResponse {
    let username = body.username.clone();
    let mut password = body.password.clone();
    let id = Uuid::new_v4();

    let user = user_exists(&app_state.pool, username.clone())
        .await
        .unwrap();
    if user {
        return HttpResponse::BadRequest().json(Response {
            message: "User already exists".to_string(),
        });
    }

    password = hash(password, DEFAULT_COST).unwrap();
    //Creates new User in database
    match sqlx::query_as!(
        User,
        "INSERT INTO user_table (user_id, user_name, password) VALUES ($1, $2, $3)",
        id,
        username,
        password
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(_) => {
            return HttpResponse::Ok().json(Response {
                message: "success".to_string(),
            })
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            })
        }
    }
}
