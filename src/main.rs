use state::AppState;
use std::env;

mod handlers;
mod payment_processor;
mod payments_dto;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    let app_state = AppState::new();

    let app = routes::init(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Server listening in 0.0.0.0:{}.", port);

    axum::serve(listener, app).await.unwrap();
}
