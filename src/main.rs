use state::AppState;
use std::env;
use std::sync::Arc;

mod handlers;
mod payment_processor;
mod payments_dto;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    let shared_state = Arc::new(AppState::new());

    let app = routes::init(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Server listening in 0.0.0.0:{}.", port);

    axum::serve(listener, app).await.unwrap();
}
