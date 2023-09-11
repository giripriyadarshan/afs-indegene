use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

mod errors;
mod routes;
mod utils;

use errors::ServerErrors;

#[tokio::main]
async fn main() -> Result<(), ServerErrors> {
    let app = Router::new()
        .route("/get-doc-type", get(handler))
        .route("/test-path", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8792));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
