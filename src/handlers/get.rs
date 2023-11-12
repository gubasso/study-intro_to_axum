use axum::extract::Path;

pub async fn root() -> &'static str {
    "Hello, World!!!!"
}

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}
