# Intro to Axum

rust axum
```rs
# at main
let app = Router::new()
    .merge(add_route());

# add route with `merge` method
fn add_route() -> Router {
    route("/", get(|| async { "Hello there!" }))
}
```
why this error: cannot find function `route` in this scope
