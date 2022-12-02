use axum::http::HeaderMap;

pub async fn get_custom_headers(headers: HeaderMap) -> String {
    let header_message = headers.get("x-message").unwrap().to_str().unwrap().to_owned();
    header_message
}