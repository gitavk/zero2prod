use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
// for `collect`
use tower::ServiceExt; // for `oneshot`

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = zero2prod::startup::app();
    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // Assert
    assert!(response.status().is_success());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = zero2prod::startup::app();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscriptions")
                .header(
                    http::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    println!("Test form {:?}", response);
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    // Act
    for (invalid_body, error_message) in test_cases {
        let app = zero2prod::startup::app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/subscriptions")
                    .header(
                        http::header::CONTENT_TYPE,
                        "application/x-www-form-urlencoded",
                    )
                    .body(Body::from(invalid_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        println!("Test form {:?}", response);
        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Bad Request when payload was {}.",
            error_message,
        );
    }
}
