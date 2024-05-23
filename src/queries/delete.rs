use sqlx::MySqlPool;

use crate::get_db_pool;

pub async fn delete(id: u64) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
    println!("DELETE QUERY. Id = {}", id);
    let pool: sqlx::Pool<sqlx::MySql> = get_db_pool().await.unwrap();
    let query = format!("DELETE FROM todos WHERE id = {}", id);
    println!("Query: {}", query);
    let rows = sqlx::query(&query).fetch_all(&pool).await;

    match rows {
        Ok(rows) => Ok(rows),
        Err(e) => Err(e),
    }
}
