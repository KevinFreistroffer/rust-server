#[path = "../queries/create.rs"]
mod add_one;
#[path = "../queries/read_all.rs"]
mod read_all;
#[path = "../structs.rs"]
mod structs;
#[path = "../queries/update.rs"]
mod update;
use self::structs::{Todo, UpdateTodo};
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use read_all::read_all;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use update::update as update_query;
// (StatusCode, HeaderMap, Json<Vec<Todo>>)

#[debug_handler]
pub async fn update(Json(payload): Json<UpdateTodo>) -> (StatusCode, HeaderMap, Json<Vec<Todo>>) {
    println!("PATCH /todo todo={:?}", payload);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    match update_query(payload).await {
        Ok(_) => {
            println!("Updated successfully");
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

            (StatusCode::CREATED, headers, Json(body))
        }
        Err(e) => {
            println!("Error: {}", e);
            (StatusCode::ALREADY_REPORTED, headers, Json(Vec::new()))
        }
    }
}
