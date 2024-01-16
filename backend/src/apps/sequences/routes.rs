use super::views;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(views::list))
        .route("/:id", get(views::detail))
}
