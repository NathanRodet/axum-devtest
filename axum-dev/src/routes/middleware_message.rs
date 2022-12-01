use axum::Extension;

use crate::middlewares::shared_data::SharedData;

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message.clone()
}