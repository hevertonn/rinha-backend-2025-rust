use crate::handlers;
use axum::{
    Router,
    routing::{get, post},
};

pub fn init_routes() -> Router {
    let routes = Router::new()
        .route("/payments", post(handlers::payments))
        .route("/payments-summary", get(handlers::payments_summary));

    routes
}
