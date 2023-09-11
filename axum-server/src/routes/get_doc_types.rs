use axum::response::Json;
use serde_json::{json, Value};

pub async fn get_doc_types(body: String) -> Json<Value> {
    // use reqwest to get from
    // https://bi.veevavault.com/api/v23.1/metadata/objects/documents/types
    // return complete response
    todo!()
}
