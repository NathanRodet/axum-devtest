use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::hello_world::hello_world;
use crate::routes::path_variables::path_variables;
use crate::routes::post_json::post_json;
use crate::routes::post_string::post_string;
use crate::routes::query_params::query_params;
use crate::routes::query_params::query_params_id;
use crate::routes::user_agent::user_agent;
use crate::routes::custom_headers::custom_headers;
use crate::middlewares::cors::create_cors;


pub async fn create_routes() -> Router {

    let cors = create_cors().await;

    Router::new()
        .route("/", get(hello_world))
        .route("/post_string", post(post_string))
        .route("/post_json", post(post_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/query_params_id", get(query_params_id))
        .route("/user_agent", get(user_agent))
        .route("/custom_headers", get(custom_headers))
        .layer(cors)
}