use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Todo {
    pub id: i64,
    // Need to be private?
    pub name: String,
    // Need to be private?
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    // Need to be private?
    pub name: String,
    // Need to be private?
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub id: i64,
    // Need to be private?
    pub name: Option<String>,
    // Need to be private?
    pub done: Option<bool>,
}
