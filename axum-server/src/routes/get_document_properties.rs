use axum::response::Json;
use serde_json::{json, Value};

pub async fn get_document_properties(body: String) -> Json<Value> {
    // parse payload to get doc_type
    // use reqwest to get from
    // https://bi.veevavault.com/api/v23.1/metadata/objects/documents/types/{doc_type}
    // check for relations
    // query id, name and value for all the relations
    // return responses including relations (file name will be parent.relation.json / parent.json)
    todo!()
}