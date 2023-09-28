use axum::response::Json;
use serde_json::{json, Value};
use std::collections::HashMap;

use reqwest::{header::AUTHORIZATION, Client};

use crate::utils::{
    encode_json::decode_json, extract_string::ExtractString, session_id::get_session_id,
};

pub async fn create_binder(body: String) -> Json<Value> {
    let body: Value = serde_json::from_str(&body).unwrap();

    let session_id = get_session_id(
        body["instance"].to_string().remove_d_quotes(),
        body["account"].to_string().remove_d_quotes(),
    )
    .await;

    let encoded_query = body["query"].to_string().remove_d_quotes();

    let decoded_query: Value = serde_json::from_str(&decode_json(encoded_query)).unwrap();

    let mut params = HashMap::new();

    for p in decoded_query.as_object().unwrap() {
        params.insert(p.0.as_str(), p.1.as_str().unwrap());
    }

    let client = Client::new();

    let res = client
        .post("https://bi.veevavault.com/api/v23.1/objects/binders")
        .header(AUTHORIZATION, session_id)
        .form(&params)
        .send()
        .await
        .unwrap();

    let res = res.json::<Value>().await.unwrap();

    Json(json!(res))
}
