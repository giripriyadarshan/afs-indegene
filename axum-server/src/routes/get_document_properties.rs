use axum::response::Json;
use serde_json::{json, Value};
use std::collections::HashMap;
use url::Url;

use reqwest::{header::AUTHORIZATION, Client};

use crate::utils::{
    encode_json::encode_json, extract_string::ExtractString, session_id::get_session_id,
};

pub async fn get_document_properties(body: String) -> Json<Value> {
    // parse payload to get doc_type
    // use reqwest to get from
    // https://bi.veevavault.com/api/v23.1/metadata/objects/documents/types/{doc_type}
    // check for relations
    // query id, name and value for all the relations
    // return responses including relations (file name will be parent.relation.json / parent.json)
    let body: Value = serde_json::from_str(&body).unwrap();

    let session_id = get_session_id(
        body["instance"].to_string().from_d_quotes(),
        body["account"].to_string().from_d_quotes(),
    )
    .await;

    let doc_type = body["doc_type"].to_string().from_d_quotes();

    let client = Client::new();
    let res = client
        .get(doc_type.as_str())
        .header(AUTHORIZATION, session_id.clone())
        .send()
        .await
        .unwrap();

    let res = res.json::<Value>().await.unwrap();

    let mut doc_type_files: HashMap<String, String> = HashMap::new();

    let main_file_name_url = Url::parse(&doc_type).unwrap();
    let main_file_name_vec = main_file_name_url
        .path_segments()
        .map(|s| s.collect::<Vec<_>>())
        .unwrap();
    let main_file_name = main_file_name_vec.last().unwrap().to_owned().to_owned();

    let properties: Value = serde_json::from_str(&res["properties"].to_string()).unwrap();
    doc_type_files.insert(main_file_name.clone(), encode_json(res.to_string()));

    if properties.is_array() {
        for prop in properties.as_array().unwrap() {
            match (
                prop["type"].as_str().unwrap(),
                prop["controllingField"].is_string(),
                prop["queryable"].as_bool().unwrap(),
                prop["systemAttribute"].as_bool().unwrap(),
                prop["disabled"].as_bool().unwrap(),
                prop["editable"].as_bool().unwrap(),
            ) {
                ("ObjectReference", true, true, false, false, true) => {
                    let sub_properties = get_sub_properties(
                        prop["objectType"].to_string().from_d_quotes(),
                        session_id.clone(),
                        true,
                        Some(prop["controllingField"].to_string().from_d_quotes()),
                    )
                    .await;
                    let file_name =
                        ".".to_owned() + prop["name"].to_string().from_d_quotes().as_str();
                    let file_name = file_name.as_str();
                    doc_type_files.insert(main_file_name.clone() + file_name, sub_properties);
                }
                ("ObjectReference", false, true, false, false, true) => {
                    let sub_properties = get_sub_properties(
                        prop["objectType"].to_string().from_d_quotes(),
                        session_id.clone(),
                        false,
                        None,
                    )
                    .await;

                    let file_name =
                        ".".to_owned() + prop["name"].to_string().from_d_quotes().as_str();
                    let file_name = file_name.as_str();
                    doc_type_files.insert(main_file_name.clone() + file_name, sub_properties);
                }
                _ => {}
            }
        }
    }

    println!("sending response");
    Json(json!(doc_type_files))
}

async fn get_sub_properties(
    name: String,
    session_id: String,
    is_controlled: bool,
    controlling_field: Option<String>,
) -> String {
    let client = Client::new();

    let mut query: String;
    if is_controlled {
        let mut relationship_name = String::new();
        let meta_url = "https://bi.veevavault.com/api/v23.1/metadata/vobjects/".to_owned() + &name;
        let meta = client
            .get(meta_url)
            .header(AUTHORIZATION, session_id.clone())
            .send()
            .await
            .unwrap();

        let meta_res = meta.json::<Value>().await.unwrap();
        let relations: Value =
            serde_json::from_str(&meta_res["object"]["relationships"].to_string()).unwrap();

        if relations.is_array() {
            for rel in relations.as_array().unwrap() {
                let rel_meta: Value = serde_json::from_str(&rel["object"].to_string()).unwrap();
                if rel_meta["name"].to_string().from_d_quotes()
                    == controlling_field.clone().unwrap()
                {
                    relationship_name = rel["relationship_name"].to_string().from_d_quotes();
                }
            }
        }
        query = String::from("SELECT id, name__v, ");
        query.push_str(&relationship_name);
        query.push('.');
        query.push_str("name__v");
        query.push_str(" FROM ");
        query.push_str(&name);
    } else {
        query = String::from("SELECT id, name__v FROM ");
        query.push_str(&name);
    }

    let mut params = HashMap::new();
    params.insert("q", query);

    let res = client
        .post("https://bi.veevavault.com/api/v23.1/query")
        .header(AUTHORIZATION, session_id)
        .form(&params)
        .send()
        .await
        .unwrap();

    let res = res.json::<Value>().await.unwrap();

    encode_json(res.to_string())
}
