use axum::{routing::get, Router};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;

mod apps;
mod urls;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // set up db connection pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;
    sqlx::migrate!().run(&pool).await?;


    // set up web server
    let app = Router::new()
        .nest_service(urls::static_dir(), ServeDir::new("static"))
        .nest_service(urls::node_modules_dir(), ServeDir::new("node_modules"))
        .nest(urls::sequences(), apps::sequences::routes::routes());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
