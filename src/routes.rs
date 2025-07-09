use std::sync::Arc;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use crate::app_state::AppState;
use crate::features::todos::routes::create_todos_routes;

pub fn create_routes() -> Router<Arc<AppState>> {
    let cors = CorsLayer::new()
        .allow_origin("0.0.0.0:3000".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    return Router::new()
        .merge(create_todos_routes())
        .route("/", get(|| async { "Welcome to the Todos API!" }))
        .layer(cors);
}