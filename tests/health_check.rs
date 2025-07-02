use std::error::Error;

use rust_practice::run_app;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    let _ = tokio::spawn(async {
        spawn_app()
            .await
            .expect("FAILED WHILE TRYING TO START A SERVER")
    });

    let cliet = reqwest::Client::new();

    let res = cliet
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

async fn spawn_app() -> Result<(), Box<dyn Error>> {
    // todo!()
    run_app().await
}
