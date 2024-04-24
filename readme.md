# Axum Masquerade

A small utility for mimicking the behavior and responses of other web technologies, to throw off attackers or just for fun!

Includes

- Apache/PHP 404, 403 pages & headers
- Express/NodeJS 404, 500 pages & headers

To return a response:

```rust
use axum_masquerade::apache_responses::ApacheNotFoundResponse;

async fn this_is_a_fake_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    ApacheNotFoundResponse::with_path(uri)
}
```

To use the headers middleware:

```rust
use axum_masquerade::headers::{HeaderType, MasqueradeHeaderLayer};

let app = Router::new()
    .route("/fake", get(handler))
    .layer(MasqueradeHeaderLayer::new(&HeaderType::ExpressNodeJs));
```
