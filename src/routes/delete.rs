#[path = "../queries/create.rs"]
mod add_one;
#[path = "../queries/delete.rs"]
mod delete;
#[path = "../queries/read_all.rs"]
mod read_all;
#[path = "../structs.rs"]
mod structs;
use crate::get_db_pool;
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use delete::delete as delete_query;
use read_all::read_all;
use serde::Deserialize;
use sqlx::mysql::MySqlRow;
use sqlx::Row;

use self::structs::Todo;

#[derive(Deserialize)]
struct Params {
    id: u64,
}

#[debug_handler]
pub async fn delete(Path(id): Path<u64>) -> (StatusCode, HeaderMap, Json<Vec<Todo>>) {
    println!("DELETE /todo id = {:?}", id);

    match delete_query(id).await {
        Ok(_) => {
            let todos = read_all().await.unwrap();
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
        Err(e) => {
            println!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                Json(Vec::new()),
            )
        }
    }
}
