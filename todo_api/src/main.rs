use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod routes;

use crate::routes::todo::todo_scope;

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
            .app_data(web::Data::new(app_state.clone()))
            .service(todo_scope())
            .route("/", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
