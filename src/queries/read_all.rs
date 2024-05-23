use sqlx::MySqlPool;

use crate::get_db_pool;

pub async fn read_all() -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
    let pool = get_db_pool().await.unwrap();
    let query = format!("SELECT * FROM todos");
    let rows = sqlx::query(&query).fetch_all(&pool).await;

    match rows {
        Ok(rows) => Ok(rows),
        Err(e) => Err(e),
    }
}
