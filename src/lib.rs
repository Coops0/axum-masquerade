pub mod apache_responses;
pub mod headers;

#[cfg(test)]
mod tests {
    use self::headers::MasqueradeHeaderLayer;

    use super::*;
    use axum_core::response::IntoResponse;
    use headers::HeaderType;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_apache_not_found_response() {
        let response = apache_responses::ApacheNotFoundResponse::with_path(String::from("/test"));
        let response = response.into_response();

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

    #[test]
    fn test_header_layer() {
        let header_layer = MasqueradeHeaderLayer::new(HeaderType::PHP);
    }
}
