use axum::{
    routing::{get, post},
    Router, http::Method,
};
use tower_http::cors::{CorsLayer, Any};

use hello_world::hello_world;
use path_variables::path_variables;
use post_json::post_json;
use post_string::post_string;
use query_params::query_params;
use query_params::query_params_id;
use user_agent::user_agent;
use custom_headers::custom_headers;

mod hello_world;
mod path_variables;
mod post_json;
mod post_string;
mod query_params;
mod user_agent;
mod custom_headers;

pub fn create_routes() -> Router {

    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);

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
