use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower::ServiceBuilder;

use crate::middlewares::cors::create_cors;
use crate::middlewares::custom_headers::middleware_read_custom_header;
use crate::middlewares::custom_headers::middleware_write_custom_header;
use crate::middlewares::shared_data::shared_message_data;
use crate::middlewares::shared_state::get_role;
use crate::routes::custom_headers::get_custom_headers;
use crate::routes::hello_world::hello_world;
use crate::routes::middleware_message::middleware_message;
use crate::routes::path_variables::get_path_variables;
use crate::routes::post_json::post_json;
use crate::routes::post_string::post_string;
use crate::routes::query_params::get_query_params;
use crate::routes::query_params::get_query_params_id;
use crate::routes::user_agent::get_user_agent;

pub async fn create_routes() -> Router {
    let cors = create_cors().await;
    let shared_data = shared_message_data().await;
    let shared_state = get_role().await;

    Router::new()
        .route(
            "/middleware_read_custom_header",
            get(middleware_read_custom_header),
        )
        .route_layer(middleware::from_fn(middleware_write_custom_header))
        .route("/", get(hello_world))
        .route("/post_string", post(post_string))
        .route("/post_json", post(post_json))
        .route("/path_variables/:id", get(get_path_variables))
        .route("/query_params", get(get_query_params))
        .route("/query_params_id", get(get_query_params_id))
        .route("/user_agent", get(get_user_agent))
        .route("/custom_headers", get(get_custom_headers))
        .route("/display_shared_data", get(middleware_message))
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(Extension(shared_data)),
        )
        .with_state(shared_state)
}
