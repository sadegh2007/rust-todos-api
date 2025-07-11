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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app_routes)
    .await
    .unwrap();
}

fn initial_state() -> Arc<AppState> {
    return Arc::new(AppState)
}