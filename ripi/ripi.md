# ripissue draft

- [ ] setup migrations (integrate SQLx with Docker init.db)
  - [ ] sqlx migration model
  - [ ] SeaORM as a model to follow?
- [x] organize myenv.sh
- [x] docker secrets

rust tokio, axum and sqlx

```rust
#[derive(Deserialize, Debug)]
pub struct NewTask {
    priority: String,
    title: String,
    description: Option<String>,
    is_default: bool,
}

pub async fn create_task(pool: Extension<PgPool>, Json(new_task): Json<NewTask>) -> Result<(), Box<dyn Error>> {
    dbg!(&new_task);
    let q = "INSERT INTO tasks (priority, title, description, is_default) VALUES ($1, $2, $3, $4)";
    query(q)
        .bind(&new_task.priority)
        .bind(&new_task.title)
        .bind(&new_task.description)
        .bind(&new_task.is_default)
        .execute(&*pool)
        .await?;
    Ok(())
}

pub async fn mirror_body_string(body: String) -> String {
    body
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", post(create_task))
        .layer(Extension(pool))
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
```

I'm getting the following error at line `.route("/tasks", post(create_task))`:

 │     the trait bound `fn(axum::Extension<sqlx::Pool<sqlx::Postgres>>, axum::Json<handlers::post::NewTask>) -> impl std::future::Future<Output = std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>>> {handlers::post::create_task}: axum::handler::Handler<_, _, _>` is not satisfied rustc (E0277) [33, 31]
 │      the following other types implement trait `axum::handler::Handler<T, S, B>`:
 │        <axum::handler::Layered<L, H, T, S, B, B2> as axum::handler::Handler<T, S, B2>>
 │        <axum::routing::MethodRouter<S, B> as axum::handler::Handler<(), S, B>>

How do I fix this? What is wrong?
