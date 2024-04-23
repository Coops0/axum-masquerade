use axum_core::response::Response;
use headers::HeaderType;
use http::{header, response::Builder, HeaderName, HeaderValue};

pub mod apache_responses;
pub mod express_response;
pub mod headers;

pub(crate) fn response_from_header(headers: &HeaderType) -> Builder {
    let mut response = Response::builder().header(header::CONTENT_TYPE, "text/html");

    extend_headers(response.headers_mut().unwrap(), headers.headers());

    response
}

pub(crate) fn extend_headers(
    headers: &mut http::HeaderMap<HeaderValue>,
    new_headers: &[(&'static str, &'static str)],
) {
    for (k, v) in new_headers {
        headers.insert(HeaderName::from_static(k), HeaderValue::from_static(v));
    }
}
