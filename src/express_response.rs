use axum_core::{body::Body, response::IntoResponse};
use http::StatusCode;

use crate::{headers::HeaderType, response_from_header};

#[derive(Debug, Clone)]
pub struct ExpressNotFoundResponse {
    path: String,
}

impl ExpressNotFoundResponse {
    #[inline]
    #[must_use]
    pub const fn with_path(path: String) -> Self {
        Self { path }
    }
}

impl IntoResponse for ExpressNotFoundResponse {
    fn into_response(self) -> axum_core::response::Response {
        let body = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/express_404.html"
        ))
        .replace("{{ url }}", &self.path);

        response_from_header(&HeaderType::Php)
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(body))
            .unwrap()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExpressInteralErrorResponse;

impl IntoResponse for ExpressInteralErrorResponse {
    fn into_response(self) -> axum_core::response::Response {
        let body = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/express_500.html"
        ));

        response_from_header(&HeaderType::Php)
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(body))
            .unwrap()
    }
}
