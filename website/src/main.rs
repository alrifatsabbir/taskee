mod assets;
mod pages;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taskee_website=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the Axum application with required routes
    let app = Router::new()
        // Primary user routes
        .route("/", get(handle_home))
        .route("/about", get(handle_about))
        .route("/install", get(handle_install))
        .route("/contact", get(handle_contact))
        // Direct Binary Downloads
        .route("/download/windows", get(handle_download_windows))
        .route("/download/taskee-windows-x86_64.exe", get(handle_download_windows))
        // Static assets (embedded in pure Rust binary)
        .route("/static/css/style.css", get(assets::serve_css))
        .route("/static/js/main.js", get(assets::serve_js))
        .route("/static/images/logo.jpg", get(assets::serve_logo))
        .route("/static/images/favicon.svg", get(assets::serve_favicon))
        .route("/favicon.svg", get(assets::serve_favicon))
        .route("/favicon.ico", get(assets::serve_favicon))
        // Health / Telemetry status API
        .route("/health", get(handle_health))
        .fallback(handle_404)
        .layer(TraceLayer::new_for_http());

    // Resolve port from environment or fallback to 3000
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener on port");

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                    TASKEE OFFICIAL WEB SERVER                     ║");
    println!("║            100% Pure Rust • Axum + Tokio Telemetry Stack          ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║  • Host:     http://localhost:{:<5}                               ║", port);
    println!("║  • Home:     http://localhost:{:<5}/                              ║", port);
    println!("║  • About:    http://localhost:{:<5}/about                         ║", port);
    println!("║  • Install:  http://localhost:{:<5}/install                       ║", port);
    println!("║  • Contact:  http://localhost:{:<5}/contact                       ║", port);
    println!("║  • Health:   http://localhost:{:<5}/health                        ║", port);
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    axum::serve(listener, app)
        .await
        .expect("Taskee web server failed to run");
}

// Handlers
async fn handle_home() -> Html<String> {
    Html(pages::home::render_home())
}

async fn handle_about() -> Html<String> {
    Html(pages::about::render_about())
}

async fn handle_install() -> Html<String> {
    Html(pages::install::render_install())
}

async fn handle_contact() -> Html<String> {
    Html(pages::contact::render_contact())
}

async fn handle_download_windows() -> Response {
    let candidate_paths = [
        "static/downloads/taskee-windows-x86_64.exe",
        "website/static/downloads/taskee-windows-x86_64.exe",
        "../target/release/taskee.exe",
        "target/release/taskee.exe",
        "../website/static/downloads/taskee-windows-x86_64.exe",
    ];

    for p in candidate_paths {
        if let Ok(bytes) = tokio::fs::read(p).await {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/vnd.microsoft.portable-executable".parse().unwrap(),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"taskee-windows-x86_64.exe\"".parse().unwrap(),
            );
            return (StatusCode::OK, headers, bytes).into_response();
        }
    }

    (
        StatusCode::NOT_FOUND,
        "Executable binary not found. Please compile taskee first with: cargo build --release --bin taskee",
    )
        .into_response()
}

async fn handle_health() -> impl IntoResponse {
    let json = r#"{"status":"ok","app":"taskee-website","rust_edition":"2024","version":"0.1.0"}"#;
    ([(header::CONTENT_TYPE, "application/json")], json)
}

async fn handle_404(req: Request<Body>) -> Response {
    let path = req.uri().path().to_string();
    (StatusCode::NOT_FOUND, Html(pages::not_found::render_not_found(&path))).into_response()
}
