use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

mod extractors;
mod routes;

use crate::routes::category::category_scope;
use crate::routes::todo::todo_scope;
use crate::routes::user::user_scope;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main()]
async fn main() -> Result<(), std::io::Error> {
    let db_url = "postgresql://api:EsqueDeathDu2711195s@localhost:5432/todo";
    println!("{}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("Error connecting to Database");
    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(String::from("secret")))
            .app_data(web::Data::new(app_state.clone()))
            .service(user_scope())
            .service(todo_scope())
            .service(category_scope())
            .route("/", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}
