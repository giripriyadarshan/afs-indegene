use axum::response::Json;
use serde_json::{json, Value};
use std::collections::HashMap;
use url::Url;

use reqwest::{header::AUTHORIZATION, Client};

use crate::utils::{
    encode_json::encode_json, extract_string::ExtractString, session_id::get_session_id,
};

pub async fn create_binder(body: String) -> String {
    let body: Value = serde_json::from_str(&body).unwrap();

    let session_id = get_session_id(
        body["instance"].to_string().remove_d_quotes(),
        body["account"].to_string().remove_d_quotes(),
    )
        .await;



    todo!()
}