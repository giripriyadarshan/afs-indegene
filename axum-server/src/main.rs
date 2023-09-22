use axum::{response::Html, routing::get, Router};
use routes::{get_doc_types::get_doc_types, get_document_properties::get_document_properties, create_binder::create_binder};
use std::net::SocketAddr;

mod errors;
mod routes;
mod utils;

use errors::ServerErrors;

#[tokio::main]
async fn main() -> Result<(), ServerErrors> {
    let app = Router::new()
        .route("/get-doc-types", get(get_doc_types))
        .route("/get-doc-properties", get(get_document_properties))
        .route("/create-binder", get(create_binder));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8792));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
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

the client will have to decode and save the files in the encoded format
later when using the jsons, the client will have to decode the jsons and use them during runtime

when sending a particular request, make sure all relations of that request are queried and sent as well
*/
