use axum::{body::Body, routing::{get, post}, Router};

mod hello_world;
mod post_string;
mod post_json;

use hello_world::hello_world;
use post_string::post_string;
use post_json::post_json;

pub fn create_routes() -> Router<Body> {
    // build our application with a single route
    Router::new()
        .route("/", get(hello_world))
        .route("/post_string", post(post_string))
        .route("/post_json", post(post_json))
}