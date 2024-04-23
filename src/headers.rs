use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum_core::{extract::Request, response::Response};
use tower::{Layer, Service};

use crate::extend_headers;

const PHP_HEADERS: &[(&str, &str)] = &[
    ("x-powered-by", "PHP/4.0.5"),
    ("server", "Apache/1.3.20 (Unix) PHP/4.0.5"),
    ("connection", "close"),
];

const EXPRESS_NODEJS_HEADERS: &[(&str, &str)] = &[
    ("x-powered-by", "Express"),
    ("x-content-type-options", "nosniff"),
];

pub struct MasqueradeHeaderLayer {
    headers: &'static [(&'static str, &'static str)],
}

impl MasqueradeHeaderLayer {
    #[must_use]
    pub const fn new(header_type: &HeaderType) -> Self {
        Self {
            headers: header_type.headers(),
        }
    }
}

pub enum HeaderType {
    Php,
    ExpressNodeJs,
}

impl HeaderType {
    #[must_use]
    pub const fn headers(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Php => PHP_HEADERS,
            Self::ExpressNodeJs => EXPRESS_NODEJS_HEADERS,
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
    headers: &'static [(&'static str, &'static str)],
    inner: S,
}

impl<S, B> Service<Request<B>> for HeaderTowerMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let headers = self.headers;
        let future = self.inner.call(request);

        Box::pin(async move {
            let mut response = future.await?;
            extend_headers(response.headers_mut(), headers);

            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use self::headers::MasqueradeHeaderLayer;

    use super::*;
    use axum::body::Body;
    use axum_core::response::Response;
    use headers::HeaderType;
    use http::{HeaderValue, Method, Request, Uri};

    use tower::{service_fn, Layer, Service};

    fn run_through_middleware<S>(
        middleware: MasqueradeHeaderLayer,
        service: S,
        req: Request<Body>,
    ) -> Pin<
        Box<
            (dyn Future<Output = Result<Response<Body>, <S as Service<Request<Body>>>::Error>>
                 + 'static),
        >,
    >
    where
        S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
        S::Future: Send,
    {
        let mut svc = middleware.layer(service);

        svc.call(req)
    }

    #[tokio::test]
    async fn masquerade_header_middleware_test() {
        let service = service_fn(|_req| async {
            Ok::<_, hyper::Error>(Response::new(Body::from("Hello World")))
        });
        let middleware = MasqueradeHeaderLayer::new(&HeaderType::Php);
        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/test"))
            .body(Body::empty())
            .unwrap();

        let response = run_through_middleware(middleware, service, req)
            .await
            .unwrap();

        let headers = response.headers();

        assert_eq!(
            headers.get("x-powered-by"),
            Some(&HeaderValue::from_static("PHP/4.0.5"))
        );
    }
}
