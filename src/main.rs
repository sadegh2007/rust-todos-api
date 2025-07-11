use std::{sync::{Arc}};
use axum::Router;
use dotenvy::dotenv;
use todos_app::{app_state::AppState, routes::create_routes};

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let shared_state: Arc<AppState> = initial_state();

    let app_routes = Router::new()
            .nest("/api", create_routes())
            .with_state(shared_state.clone());

    // Use the environment variables for host and port if available
    let server_url = format!("{}:{}", 
        std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()), 
        std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string())
    );

    let listener = tokio::net::TcpListener::bind(server_url)
        .await
        .unwrap();

    axum::serve(listener, app_routes)
    .await
    .unwrap();
}

fn initial_state() -> Arc<AppState> {
    return Arc::new(AppState)
}