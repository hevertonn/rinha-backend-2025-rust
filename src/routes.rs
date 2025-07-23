use axum::{
    Router,
    routing::{get, post},
};

pub fn init_routes() -> Router {
    let routes = Router::new()
        .route("/payments", post(|| async { "payments" }))
        .route("/payments-summary", get(|| async { "payments-summary" }));

    routes
}
