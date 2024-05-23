use crate::get_db_pool;

use super::structs::CreateTodo;
use sqlx::MySqlPool;

pub async fn create(todo: CreateTodo) -> anyhow::Result<u64> {
    let pool = get_db_pool().await.unwrap();
    let query = format!(
        r#"INSERT INTO todos ( name, done ) VALUES ( "{}", {} )"#,
        todo.name, todo.done
    );
    println!("Query: {}", query);
    let result = sqlx::query(&query).execute(&pool).await?.last_insert_id();
    println!("result: {}", result);
    return Ok(result);
}
