use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
  id: String,
  title: String,
  completed: bool
}

impl Todo {
  pub fn new(title: &str, completed: Option<bool>) -> Todo {
    Todo {
      id: Uuid::new_v4().to_string(),
      title: String::from(title),
      completed: match completed {
        Some(completed) => completed,
        None => false,
      },
    }
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn completed(&self) -> bool {
    self.completed
  }

  pub fn set_completed(&mut self, completed: bool) {
    self.completed = completed;
  }
}