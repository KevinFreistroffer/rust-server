#[path = "../queries/create.rs"]
mod create;
#[path = "../queries/read_all.rs"]
mod read_all;
#[path = "../structs.rs"]
mod structs;
use self::structs::Todo;
use axum::http::HeaderMap;
use axum::{http::StatusCode, Json};
use axum_macros::debug_handler;
use create::create as create_query;
use read_all::read_all;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use structs::CreateTodo;

#[debug_handler]
pub async fn create(Json(payload): Json<CreateTodo>) -> (StatusCode, HeaderMap, Json<Vec<Todo>>) {
    println!("/todo: {:?}", payload);

    let todo = CreateTodo {
        name: payload.name,
        done: payload.done,
    };

    match create_query(todo).await {
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
