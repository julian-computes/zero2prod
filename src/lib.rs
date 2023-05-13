pub mod configuration;
pub mod prelude;
pub mod routes;
pub mod startup;

use axum::routing::{get, post, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;

use std::net::SocketAddr;
use std::sync::Arc;

use routes::{health_check, subscribe};
use sqlx::PgPool;

pub struct TestThing {}

pub fn run(port: u16, connection: PgPool) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let connection = Arc::new(connection);
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr).serve(app.into_make_service())
}
