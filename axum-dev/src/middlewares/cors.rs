use axum::{
    http::Method,
};
use tower_http::cors::{CorsLayer, Any};

pub async fn create_cors() -> CorsLayer {

    CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)

}