use actix_web::{cookie::Cookie, web, HttpResponse, Scope};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::AppState;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/sign-up", web::post().to(user_signup))
        .route("/login", web::post().to(user_login))
        .route("/logout", web::get().to(user_logout))
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
    user_id: Uuid,
    user_name: String,
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

async fn user_login(
    body: web::Json<Login>,
    app_state: web::Data<AppState>,
    secret: web::Data<String>,
) -> HttpResponse {
    let username = body.username.clone();
    let password = body.password.clone();

    //Check if user exists
    let user = query_as!(
        User,
        "SELECT user_id, user_name, password FROM user_table WHERE user_name = $1",
        username
    )
    .fetch_one(&app_state.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::BadRequest().json(Response {
                message: format!("User with username: {} does not exist", username),
            })
        }
    };

    let verified = verify(password, &user.password);
    let user_id = user.user_id.clone();

    //Check if password matches
    match verified {
        Ok(is_password_correct) => {
            if !is_password_correct {
                return HttpResponse::BadRequest().json(Response {
                    message: "Wrong password or username".to_string(),
                });
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            })
        }
    }

    let exp = (Utc::now() + Duration::try_hours(8).expect("Error calculating Duration")).timestamp()
        as usize;
    let claims = Claims { user_id, exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("todo_auth", token)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(Response {
        message: "Successfully logged in".to_string(),
    })
}

async fn user_logout() -> HttpResponse {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("todo_auth", "")
                .path("/")
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish(),
        )
        .json(Response {
            message: "Logged out successfully".to_string(),
        })
}
