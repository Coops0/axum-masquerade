use axum::{extract::OriginalUri, routing::get, Router};
use axum_masquerade::{
    apache_responses::{ApacheForbiddenResponse, ApacheNotFoundResponse},
    express_response::{ExpressInteralErrorResponse, ExpressNotFoundResponse},
    headers::{HeaderType, MasqueradeHeaderLayer},
};

#[tokio::main]
async fn main() {
    // lovely route layer applies to ALL routes above :)
    let headers_router = Router::new()
        .route("/express", get(|| async { "hi" }))
        .route_layer(MasqueradeHeaderLayer::new(&HeaderType::ExpressNodeJs));
    // .route("/php", get(|| async { "hi" }))
    // .route_layer(MasqueradeHeaderLayer::new(&HeaderType::Php));

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/forbidden",
            get(|| async { ApacheForbiddenResponse::with_path("/forbidden") }),
        )
        .route(
            "/express_error",
            get(|| async { ExpressInteralErrorResponse }),
        )
        .route(
            "/express_not_found",
            get(|| async { ExpressNotFoundResponse::with_path("/express_not_found") }),
        )
        .nest("/headers", headers_router)
        .fallback(|OriginalUri(uri): OriginalUri| async {
            ApacheNotFoundResponse::with_path(uri) // this takes impl ToString which uri implements
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
