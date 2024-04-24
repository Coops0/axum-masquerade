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
    pub fn with_path<S: ToString>(path: S) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl IntoResponse for ExpressNotFoundResponse {
    fn into_response(self) -> axum_core::response::Response {
        let body = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/express_404.html"
        ))
        .replace("{{ url }}", &self.path);

        response_from_header(&HeaderType::ExpressNodeJs)
            .status(StatusCode::NOT_FOUND)
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

        response_from_header(&HeaderType::ExpressNodeJs)
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(body))
            .unwrap()
    }
}
