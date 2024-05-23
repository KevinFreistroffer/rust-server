use super::structs::UpdateTodo;
use crate::get_db_pool;

pub async fn update(payload: UpdateTodo) -> Result<(), sqlx::Error> {
    let pool = get_db_pool().await.unwrap();
    let mut query = String::from("UPDATE todos SET ");
    let id: Option<i64> = Some(payload.id);
    let name = payload.name;
    let done = payload.done;
    let include_comma = !name.is_none() && !done.is_none();
    if !name.is_none() {
        query.push_str(&format!(r#"name = "{}""#, name.unwrap()));
    }
    if include_comma {
        query.push_str(", ");
    }
    if !done.is_none() {
        query.push_str(&format!(" done = {} ", done.unwrap()));
    }
    query.push_str(&format!(r#"WHERE id = {}"#, id.unwrap()));
    println!("Query: {}", query);
    let response = sqlx::query(&query).execute(&pool).await?;
    let rows_affected: u64 = response.rows_affected();
    println!("Rows affected: {}", rows_affected);
    // let response = sqlx::query(&query).fetch_all(&pool).await;
    print!("Response: {:?}", response);

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    } else {
        return Ok(());
    }
}
