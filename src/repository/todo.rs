use std::convert::From;
use sqlx::{Pool, Postgres};

use crate::model::todo::Todo;

pub enum TodoRepositoryError {
  TodoNotFoundError,
  DatabaseError(sqlx::Error)
}

impl From<sqlx::Error> for TodoRepositoryError {
  fn from(err: sqlx::Error) -> Self {
    match err {
      sqlx::Error::RowNotFound => TodoRepositoryError::TodoNotFoundError,
      _ => TodoRepositoryError::DatabaseError(err)
    }
  }
}

pub struct TodoRepository {
  conn: Pool<Postgres>
}

impl TodoRepository {
  pub fn new(conn: Pool<Postgres>) -> TodoRepository {
    TodoRepository {
      conn
    }
  }

  pub async fn get(&self, id: &str) -> Result<Todo, TodoRepositoryError> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
      .bind(id)
      .fetch_one(&self.conn)
      .await.map_err(Into::into)
  }

  pub async fn list(&self) -> Result<Vec<Todo>, TodoRepositoryError> {
    sqlx::query_as::<_, Todo>("SELECT * from todos").fetch_all(&self.conn).await.map_err(Into::into)
  }

  pub async fn insert(&self, todo: &Todo) -> Result<Todo, TodoRepositoryError> {
    sqlx::query_as::<_, Todo>("INSERT INTO todos (id, title, completed) VALUES ($1, $2, $3) RETURNING *")
        .bind(todo.id())
        .bind(todo.title())
        .bind(todo.completed())
        .fetch_one(&self.conn)
        .await.map_err(Into::into)
  }

  pub async fn update(&self, todo: &Todo) -> Result<Todo, TodoRepositoryError> {
    sqlx::query_as::<_, Todo>("UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *")
      .bind(todo.title())
      .bind(todo.completed())
      .bind(todo.id())
      .fetch_one(&self.conn)
      .await.map_err(Into::into)
  }
}