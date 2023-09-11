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

/*
basically return all files in json for the client to save in a file

the format will be
{
    "file-name": "{
        "json as string": "possible encode so that it wont be confused with values"
    }",
    "file-name2": "{same way encoded json}"
}

the client will have to decode and save the files in the correct format

when sending a particular request, make sure all relations of that request are queried and sent as well
*/
