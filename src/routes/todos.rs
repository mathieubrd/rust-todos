use actix_web::{get, post, put, web};
use serde::{Deserialize, Serialize};

use crate::{model::todo::Todo, AppState};

#[derive(Deserialize, Serialize)]
struct CreateTodoParams {
    title: String,
    completed: Option<bool>,
}

#[derive(Serialize)]
struct GetTodosResponse {
    todos: Vec<Todo>,
}

#[get("/todos")]
async fn get_todos(app_state: web::Data<AppState>) -> Result<web::Json<GetTodosResponse>, actix_web::Error> {
    app_state.todo_repository.list().await.map(|todos| web::Json(GetTodosResponse { todos: todos.to_vec() })).map_err(Into::into)
}

#[get("/todos/{id}")]
async fn get_todo(app_state: web::Data<AppState>, id: web::Path<String>) -> Result<web::Json<Todo>, actix_web::Error> {
    let id = id.into_inner();
    app_state.todo_repository.get(&id).await.map(|todo| web::Json(todo)).map_err(Into::into)
}

#[post("/todos")]
async fn create_todo(app_state: web::Data<AppState>, todo: web::Json<CreateTodoParams>) -> Result<web::Json<Todo>, actix_web::Error> {
    app_state.todo_repository.insert(&Todo::new(&todo.title, todo.completed)).await.map(|todo| web::Json(todo)).map_err(Into::into)
}


#[put("/todos/{id}/complete")]
async fn complete_todo(app_state: web::Data<AppState>, id: web::Path<String>) -> Result<web::Json<Todo>, actix_web::Error> {
    let id = id.into_inner();
    let todo = app_state.todo_repository.get(&id).await;

    match todo {
        Ok(mut todo) => {
            todo.set_completed(true);
            app_state.todo_repository.update(&todo).await.map(|todo| web::Json(todo)).map_err(Into::into)
        },

        Err(err) => Err(err.into())
    }
}