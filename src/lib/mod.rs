use std::error::Error;

use axum::{Router, response::IntoResponse, routing::get, serve::Listener};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn health_check() {}

async fn always_error() -> Result<String, ()> {
    Err(())
}

pub async fn run_app(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/experiment", get(RouteHandler::experiment))
        .route("/error", get(always_error));
    // let port = 8080;
    // let listener = tokio::net::TcpListener::bind(addr).await?;
    // println!("Listening on address : {addr}");
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpRequest {
    email: String,
    password: String,
}

// impl SignUpResponse {

// }

impl Default for SignUpRequest {
    fn default() -> Self {
        Self {
            email: "arbaz.irshad@sparkosol.com".to_owned(),
            password: "12345678Aa".to_string(),
        }
    }
}
