use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum_core::{extract::Request, response::Response};
use http::{header, HeaderName, HeaderValue};
use tower::{Layer, Service};

// todo why does this have to be static & not const
static PHP_HEADERS: [(HeaderName, HeaderValue); 3] = [
    (
        HeaderName::from_static("x-powered-by"),
        HeaderValue::from_static("PHP/4.0.5"),
    ),
    (
        header::SERVER,
        HeaderValue::from_static("Apache/1.3.20 (Unix) PHP/4.0.5"),
    ),
    (header::CONNECTION, HeaderValue::from_static("close")),
];

pub struct MasqueradeHeaderLayer {
    headers: &'static [(HeaderName, HeaderValue)],
}

impl MasqueradeHeaderLayer {
    pub fn new(header_type: HeaderType) -> Self {
        Self {
            headers: header_type.headers(),
        }
    }
}

pub enum HeaderType {
    PHP,
}

impl HeaderType {
    pub fn headers(&self) -> &'static [(HeaderName, HeaderValue)] {
        match self {
            HeaderType::PHP => &PHP_HEADERS,
        }
    }
}

impl<S> Layer<S> for MasqueradeHeaderLayer {
    type Service = HeaderTowerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HeaderTowerMiddleware {
            headers: self.headers,
            inner,
        }
    }
}

pub struct HeaderTowerMiddleware<S> {
    headers: &'static [(HeaderName, HeaderValue)],
    inner: S,
}

impl<S> Service<Request> for HeaderTowerMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let headers = self.headers.to_vec();
        let future = self.inner.call(request);

        Box::pin(async move {
            let mut response: Response = future.await?;
            response.headers_mut().extend(headers);
            Ok(response)
        })
    }
}
