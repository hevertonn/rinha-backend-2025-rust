use crate::{handlers, state::AppState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn init_routes(app_state: AppState) -> Router {
    let routes = Router::new()
        .route("/payments", post(handlers::payments))
        .route("/payments-summary", get(handlers::payments_summary))
        .with_state(app_state);

    routes
}
