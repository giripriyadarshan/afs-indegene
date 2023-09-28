use axum::response::Json;
use serde_json::{json, Value};

use reqwest::{header::AUTHORIZATION, Client};

use crate::utils::{
    extract_string::ExtractString, get_instance_url::get_instance_url, session_id::get_session_id,
};

pub async fn get_doc_types(body: String) -> Json<Value> {
    // use reqwest to get from
    // https://bi.veevavault.com/api/v23.1/metadata/objects/documents/types
    // return complete response

    let body: Value = serde_json::from_str(&body).unwrap();

    let session_id = get_session_id(
        body["instance"].to_string().remove_d_quotes(),
        body["account"].to_string().remove_d_quotes(),
    )
    .await;
    let vault_url = format!(
        "{}api/v23.1/metadata/objects/documents/types",
        get_instance_url(body["instance"].to_string().remove_d_quotes())
    );

    let client = Client::new();
    let res = client
        .get(vault_url.as_str())
        .header(AUTHORIZATION, session_id)
        .send()
        .await
        .unwrap();

    let res = res.json::<Value>().await.unwrap();

    Json(json!(res))
}
