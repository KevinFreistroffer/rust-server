#[path = "../queries/read_all.rs"]
mod read_all_query;
#[path = "../structs.rs"]
mod structs;
use axum::http::HeaderMap;
use axum::{http::StatusCode, Json};
use axum_macros::debug_handler;
use read_all_query::read_all as query_read_all;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use structs::Todo;

#[debug_handler]
pub async fn read_all() -> (StatusCode, HeaderMap, Json<Vec<Todo>>) {
    println!("GET /todos");
    let todos = query_read_all().await.unwrap();
    let json = Json(todos);
    let mut body: Vec<Todo> = Vec::new();
    json.iter().for_each(|row: &MySqlRow| {
        let todo = Todo {
            id: row.get(0),
            name: row.get(1),
            done: row.get(2),
        };
        println!("todo: {:?}", todo);
        body.push(todo);
    });

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    (StatusCode::CREATED, headers, Json(body))
}
