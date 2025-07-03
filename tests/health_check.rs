use std::error::Error;

use axum::{http::request, serve::Listener};
use rust_practice::{SignUpRequest, run_app};
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
