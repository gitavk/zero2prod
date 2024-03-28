use axum::{
    routing::get,
    Router,
};
use axum::extract::Path;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet(
    Path(params): Path<HashMap<String, String>>,
    ) -> String {
    let default = "World".to_string();
    let name = params.get("name").unwrap_or(&default);
    format!("hello {}", name)
}
