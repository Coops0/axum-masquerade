use axum_core::{
    body::Body,
    response::{IntoResponse, Response},
};
use http::{header, StatusCode};

#[derive(Default, Debug, Clone)]
pub struct ApacheNotFoundResponse {
    path: Option<String>,
}

impl ApacheNotFoundResponse {
    #[inline]
    pub fn with_path(path: String) -> Self {
        Self { path: Some(path) }
    }
}

impl IntoResponse for ApacheNotFoundResponse {
    fn into_response(self) -> axum_core::response::Response {
        let path = self.path.map(|p| format!(" {p}")).unwrap_or_default();

        let body = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/apache_404.html"
        ))
        .replace("{{ url }}", &path);

        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(body))
            .unwrap()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ApacheForbiddenResponse {
    path: Option<String>,
}

impl ApacheForbiddenResponse {
    #[inline]
    pub fn with_path(path: String) -> Self {
        Self { path: Some(path) }
    }
}

impl IntoResponse for ApacheForbiddenResponse {
    fn into_response(self) -> axum_core::response::Response {
        let path = self.path.unwrap_or_else(|| String::from("this resource"));

        let body = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/apache_403.html"
        ))
        .replace("{{ url }}", &path);

        Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(body))
            .unwrap()
    }
}
