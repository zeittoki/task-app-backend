use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, FromRow, Deserialize)]
pub struct NewTask {
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub completed: Option<bool>,
}
