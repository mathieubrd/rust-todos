mod model;
mod repository;
mod routes;

use dotenv::dotenv;
use repository::todo::TodoRepositoryError;
use std::env;

use crate::repository::todo::TodoRepository;

use actix_web::{middleware::{self, Logger}, web, App, HttpServer, Result};

use sqlx::postgres::PgPoolOptions;

struct AppState {
    todo_repository: TodoRepository
}

impl From<TodoRepositoryError> for actix_web::error::Error {
    fn from(err: TodoRepositoryError) -> Self {
        match err {
            TodoRepositoryError::TodoNotFoundError => actix_web::error::ErrorNotFound(format!("Todo not found")),
            TodoRepositoryError::DatabaseError(err) => actix_web::error::ErrorInternalServerError(err)
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "DATABASE_URL must be set"))?;
    
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

    println!("Running SQL migrations...");

    sqlx::migrate!().run(&pool).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let app_state = web::Data::new(AppState {
        todo_repository: TodoRepository::new(pool)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(routes::todos::complete_todo)
            .service(routes::todos::get_todo)
            .service(routes::todos::get_todos)
            .service(routes::todos::create_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}