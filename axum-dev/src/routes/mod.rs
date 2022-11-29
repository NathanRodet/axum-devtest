use axum::{
    routing::{get, post},
    Router,
};

mod hello_world;
mod path_variables;
mod post_json;
mod post_string;
mod query_params;

use hello_world::hello_world;
use path_variables::path_variables;
use post_json::post_json;
use post_string::post_string;
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/post_string", post(post_string))
        .route("/post_json", post(post_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
}
