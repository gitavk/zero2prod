use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
async fn subscribe(Form(form): Form<FormData>) -> String {
    format!("Welcome {}! Will sent news to {}", form.name, form.email)
}
async fn health_check() -> StatusCode {
    StatusCode::OK
}

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
