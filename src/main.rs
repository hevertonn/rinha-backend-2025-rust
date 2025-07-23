use std::env;

mod handlers;
mod payments_dto;
mod routes;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    let app = routes::init_routes();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Server listening in 0.0.0.0:{}.", port);

    axum::serve(listener, app).await.unwrap();
}
