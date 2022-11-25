use axum::{body::Body, routing::{get, post}, Router};

mod hello_world;
mod post_string;
mod post_json;
mod path_variables;

use hello_world::hello_world;
use post_string::post_string;
use post_json::post_json;
use path_variables::path_variables;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/post_string", post(post_string))
        .route("/post_json", post(post_json))
        .route("/path_variables/:id", get(path_variables))
}