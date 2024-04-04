use crate::routers::{health_check, subscribe};
use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    // build our application with a route
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run() -> () {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let app = app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
