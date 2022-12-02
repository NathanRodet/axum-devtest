use axum::{
    http::{header::ToStrError, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

#[derive(Clone)]
pub struct HeaderMessage(String);

pub async fn middleware_read_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn middleware_write_custom_header<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let header_message = headers
        .get("x-message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let header_message = header_message
        .to_str()
        .map_err(|_error: ToStrError| StatusCode::BAD_REQUEST)?
        .to_owned();

    let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(header_message.to_owned()));
    Ok(next.run(req).await)
}
