use axum_core::{body::Body, response::IntoResponse};
use http::StatusCode;

use crate::{headers::HeaderType, response_from_header};

#[derive(Default, Debug, Clone)]
pub struct ApacheNotFoundResponse {
    path: Option<String>,
}

impl ApacheNotFoundResponse {
    #[inline]
    #[must_use]
    pub fn with_path<S: ToString>(path: S) -> Self {
        Self {
            path: Some(path.to_string()),
        }
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

        response_from_header(&HeaderType::Php)
            .status(StatusCode::NOT_FOUND)
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
    #[must_use]
    pub fn with_path<S: ToString>(path: S) -> Self {
        Self {
            path: Some(path.to_string()),
        }
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

        response_from_header(&HeaderType::Php)
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(body))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use http::{header, HeaderValue};
    use http_body_util::BodyExt;

    use crate::apache_responses::ApacheNotFoundResponse;

    #[tokio::test]
    async fn test_apache_not_found_response() {
        let response = ApacheNotFoundResponse::with_path(String::from("/test"));
        let response = response.into_response();

        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/html"))
        );

        let body = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .into(),
        )
        .unwrap();

        let our_body = r#"<!DOCTYPE html PUBLIC "-//IETF//DTD HTML 2.0//EN">
        <html>
            <head>
                <title>404 Not Found</title>
            </head>
            <body>
                <h1>Not Found</h1>
                <p>The requested URL /test was not found on this server.</p>
                <hr />
                <address>Apache/1.3.20 Server at localhost Port 80</address>
            </body>
        </html>"#;

        // dumb formatting
        assert_eq!(
            body.replace("\n", "").replace(" ", ""),
            our_body.replace("\n", "").replace(" ", "")
        );
    }
}
