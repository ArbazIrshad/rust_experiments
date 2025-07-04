use std::error::Error;

use axum::{http::request, serve::Listener};
use rust_practice::{SignUpRequest, run_app};
use serde::Serialize;
use tokio::net::TcpListener;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // let _ = tokio::spawn(async {
    //     spawn_app()
    //         .await
    //         .expect("FAILED WHILE TRYING TO START A SERVER")
    // });

    let addr = spawn_app().await;

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn signup_returns_200() {
    let addr = spawn_app().await;

    let client = reqwest::Client::new();
    let body = SignUpRequest::default();
    let res = client
        .post(&format!("{}/signup", &addr))
        .json(&body)
        .send()
        .await
        .expect("Failed the sign up request");

    assert_eq!(200, res.status().as_u16())
}

#[derive(Debug, Serialize)]
struct SignUpRequestBody {
    email: Option<String>,
    password: Option<String>,
    error_message: String,
}

#[tokio::test]
async fn signup_returns_400_when_result_is_missing() {
    let addr = spawn_app().await;

    let client = reqwest::Client::new();

    let body = vec![
        SignUpRequestBody {
            email: None,
            password: None,
            error_message: "Email and password is missing".to_string(),
        },
        SignUpRequestBody {
            email: None,
            password: Some("password123".to_string()),
            error_message: "email is missing".to_string(),
        },
        SignUpRequestBody {
            email: Some(""),
            password: None,
            error_message: "".to_string(),
        },
    ];

    for bo in &body {
        let res = client
            .post("")
            .json(bo)
            .send()
            .await
            .expect("Failed to execute the request");

        assert_eq!(400, res.status().as_u16(),)
    }
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Couln't bind to the port");
    // todo!()
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(async {
        run_app(listener)
            .await
            .expect("Failed While trying to start a server")
        // spawn_app()
        // .await
        // .expect("FAILED WHILE TRYING TO START A SERVER")
    });

    format!("http://localhost:{port}")
}
