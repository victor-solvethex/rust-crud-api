use chrono::prelude::*;
use mobc_postgres::tokio_postgres::Row;
use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct SearchQuery {
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
}

impl From<Row> for Todo {
    fn from(row: Row) -> Self {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let created_at: DateTime<Utc> = row.get(2);
        let checked: bool = row.get(3);
        Todo {
            id,
            name,
            created_at,
            checked,
        }
    }
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct TodoUpdateRequest {
    pub name: String,
    pub checked: bool,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: i32,
    pub name: String,
    pub checked: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            name: todo.name,
            checked: todo.checked,
        }
    }
}
