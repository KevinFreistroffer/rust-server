#[path = "./configs.rs"]
mod configs;
#[path = "./structs.rs"]
mod structs;
// Routes
#[path = "./routes/create.rs"]
mod create;
#[path = "./routes/delete.rs"]
mod delete;
#[path = "./routes/fallback.rs"]
mod fallback;
#[path = "./routes/read_all.rs"]
mod read_all;
#[path = "./routes/root.rs"]
mod root;
#[path = "./routes/update.rs"]
mod update;
// Queries
#[path = "./queries/read_all.rs"]
mod read_all_query;
use configs::DATABASE_URL;
use create::create;
use delete::delete as delete_route;
use fallback::fallback;
use http::{header, Method, Request, Response};
use read_all::read_all;
use root::root;
use sqlx::mysql::{MySql, MySqlPool};
use tower::{Service, ServiceBuilder, ServiceExt};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use update::update;
// use sqlx::Error;
use axum::{
    extract::State,
    routing::{delete, get, patch, post},
    Extension, Router,
};
use sqlx::Pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DATABASE_URL: {}", DATABASE_URL);

    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(read_all))
        .route("/todo", post(create))
        .route("/todo/:id", delete(delete_route))
        .route("/todo", patch(update))
        .fallback(fallback)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    print!("Connected to database: ");

    Ok(())
}

pub async fn get_db_pool() -> Result<Pool<MySql>, sqlx::Error> {
    let pool: Result<Pool<MySql>, sqlx::Error> = MySqlPool::connect(DATABASE_URL).await;

    match pool {
        Ok(pool) => Ok(pool),
        // Error::PoolClosed  => println!("Connected to database"),
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }
}
