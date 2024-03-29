use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;
use toml::Table;
use url::Url;

use reqwest::{header::AUTHORIZATION, Client};

use crate::utils::send_status_message::send_message;

pub async fn get_keymessage_id(
    run_code: Arc<String>,
    binder_id: String,
    veeva_link: String,
    session_id: String,
    shared_folder_id: String,
) -> Table {
    let veeva_url = Url::parse(veeva_link.as_str()).unwrap();

    let veeva_url = format!("{}://{}", veeva_url.scheme(), veeva_url.host_str().unwrap());

    // check if key_messages_id.toml file exists and is empty
    if !std::path::Path::new("key_messages_id.toml").exists()
        || std::path::Path::new("key_messages_id.toml")
            .metadata()
            .unwrap()
            .len()
            == 0
    {
        return generate_new_keymessage_id(
            run_code,
            binder_id,
            veeva_url,
            session_id,
            shared_folder_id,
        )
        .await;
    }

    // read key_messages_id.toml file
    let file = File::open("key_messages_id.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    toml::from_str(contents.as_str()).unwrap()
}

async fn generate_new_keymessage_id(
    run_code: Arc<String>,
    binder_id: String,
    veeva_url: String,
    session_id: String,
    shared_folder_id: String,
) -> Table {
    let client = Client::new();
    let res = client
        .get(format!("{}/api/v23.1/objects/binders/{}", veeva_url, binder_id).as_str())
        .header(AUTHORIZATION, session_id.clone())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        let body = res.json::<Value>().await.unwrap();
        let nodes = body["binder"]["nodes"].as_array().unwrap();
        let mut key_messages_id_table = Table::new();
        for node in nodes {
            let key_message_name = node["properties"]["name__v"].as_str().unwrap();
            let key_message_id = node["properties"]["document_id__v"]
                .as_u64()
                .unwrap()
                .to_string();
            key_messages_id_table.insert(
                key_message_name.to_owned(),
                toml::Value::String(key_message_id),
            );
        }

        let shared_res = Client::new()
            .get(
                format!(
                    "{}/api/v23.1/objects/documents/{}",
                    veeva_url,
                    shared_folder_id.clone()
                )
                .as_str(),
            )
            .header(AUTHORIZATION, session_id)
            .send()
            .await
            .unwrap();

        if shared_res.status().is_success() {
            let shared_body = shared_res.json::<Value>().await.unwrap();
            let key_message_name = shared_body["document"]["name__v"].as_str().unwrap();
            let key_message_id = shared_folder_id;
            key_messages_id_table.insert(
                key_message_name.to_owned(),
                toml::Value::String(key_message_id),
            );
        } else {
            send_message(
                run_code,
                format!(
                    "DEV | FAILED | invalid shared folder id while getting keymessages: {}",
                    shared_folder_id
                ),
            )
            .await;
            std::process::exit(1);
        }

        // check if key_messages_id.toml file exists
        if !std::path::Path::new("key_messages_id.toml").exists() {
            File::create("key_messages_id.toml").unwrap();
        }

        // write key_messages_id.toml file
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open("key_messages_id.toml")
            .unwrap();
        file.write_all(toml::to_string(&key_messages_id_table).unwrap().as_bytes())
            .unwrap();
        key_messages_id_table
    } else {
        send_message(
            run_code,
            "DEV | FAILED | invalid session id while getting keymessages".to_owned(),
        )
        .await;
        std::process::exit(1);
    }
}
