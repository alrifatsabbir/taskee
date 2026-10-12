use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

pub const STYLE_CSS: &str = include_str!("../static/css/style.css");
pub const MAIN_JS: &str = include_str!("../static/js/main.js");
pub const FAVICON_SVG: &str = include_str!("../static/images/favicon.svg");
pub const LOGO_JPG: &[u8] = include_bytes!("../static/images/logo.jpg");

pub async fn serve_css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css; charset=utf-8".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=3600".parse().unwrap());
    (StatusCode::OK, headers, STYLE_CSS)
}

pub async fn serve_js() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/javascript; charset=utf-8".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=3600".parse().unwrap());
    (StatusCode::OK, headers, MAIN_JS)
}

pub async fn serve_favicon() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=86400".parse().unwrap());
    (StatusCode::OK, headers, FAVICON_SVG)
}

pub async fn serve_logo() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=86400".parse().unwrap());
    (StatusCode::OK, headers, LOGO_JPG)
}
