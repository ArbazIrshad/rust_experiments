use std::error::Error;

use axum::{Router, response::IntoResponse, routing::get};

async fn health_check() {}

async fn always_error() -> Result<String, ()> {
    Err(())
}

pub async fn run_app() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/experiment", get(RouteHandler::experiment))
        .route("/error", get(always_error));
    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Listening on Port : {port}");
    let server = axum::serve(listener, router).await?;
    // ()
    Ok(server)
}

pub enum AppError {
    unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

pub struct RouteHandler;

impl RouteHandler {
    pub async fn experiment() -> impl IntoResponse {
        "Experiment Successfull"
    }
}
