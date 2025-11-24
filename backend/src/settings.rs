use axum::{routing::get, Router};


#[tokio::main]
pub async fn serve() {
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/hf2p", get(|| async {"Hello from second path"}))
    .route("/", get(|| async { "Hello, World!" }))
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();

}



