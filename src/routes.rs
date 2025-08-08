use crate::{handlers, state::SharedState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn init(shared_state: SharedState) -> Router {
    let routes = Router::new()
        .route("/payments", post(handlers::payments))
        .route("/payments-summary", get(handlers::payments_summary))
        .with_state(shared_state);

    routes
}
