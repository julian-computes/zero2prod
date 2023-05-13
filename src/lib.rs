pub mod prelude;

use axum::routing::{get, post, IntoMakeService};
use axum::{Form, Router, Server};
use hyper::server::conn::AddrIncoming;

use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use std::net::SocketAddr;

pub mod configuration;
pub mod routes;
pub mod startup;

async fn health_check() {}

#[derive(Deserialize)]
struct SubscribeForm {
    name: String,
    email: String,
}

async fn subscribe(form: Result<Form<SubscribeForm>, FormRejection>) -> impl IntoResponse {
    match form {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub fn run(port: u16) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr).serve(app.into_make_service())
}
