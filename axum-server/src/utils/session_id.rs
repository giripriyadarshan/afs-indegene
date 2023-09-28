use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml::Table;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use crate::utils::get_instance_url::get_instance_url;

pub async fn get_session_id(instance: impl Into<String>, account: impl Into<String>) -> String {
    // read from sessions.toml file
    // check if the session id is valid (check_session_id())
    // if valid, return the session id
    // if not valid, renew the session id (renew_session_id())
    // return the session id

    let instance = instance.into();
    let account = account.into();
    // check if session_id.toml file exists and is empty
    if !std::path::Path::new("./session_id.toml").exists()
        || std::path::Path::new("./session_id.toml")
            .metadata()
            .unwrap()
            .len()
            == 0
    {
        return renew_session_id(instance, account).await;
    }

    let file = File::open("./session_id.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let session_id_table: Table = toml::from_str(contents.as_str()).unwrap();
    let session_id = session_id_table[instance.as_str()][account.as_str()]
        .as_str()
        .unwrap();

    // check if session_id is valid
    let client = Client::new();
    let res = client
        .post(
            format!(
                "{}/api/v23.1/keep-alive",
                get_instance_url(instance.clone())
            )
            .as_str(),
        )
        .header(AUTHORIZATION, session_id.clone())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        // read response
        let body = res.text().await.unwrap();
        if body.contains("FAILURE") {
            renew_session_id(instance, account).await
        } else {
            session_id.to_owned()
        }
    } else {
        eprintln!("Error: {}", res.status());
        std::process::exit(1);
    }
}

async fn renew_session_id(instance: impl Into<String>, account: impl Into<String>) -> String {
    // get password from zoho_auth.rs
    // decode the password using secret key from $ZOHO_SECRET_KEY
    // renew the session id
    // write to sessions.toml file
    // return the session id

    let instance = instance.into();

    let client = Client::new();
    let res = client
        .post(format!("{}/api/v23.1/auth", get_instance_url(instance.clone())).as_str())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(ACCEPT, "application/json")
        .body("username=Indegene_BI_US@bi-vault.com&password=Veeva2023@b")
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        let value = res.json::<Value>().await.unwrap();
        let session_id = value["sessionId"].as_str().unwrap();
        // check if session_id.toml file exists
        let mut file: File;
        if !std::path::Path::new("./session_id.toml").exists() {
            // create session_id.toml file
            file = File::create("./session_id.toml").unwrap();
        } else {
            file = File::open("./session_id.toml").unwrap();
        }

        // parse session_id.toml file
        let mut contents = String::new();
        let mut session_id_toml = Table::new();
        // check if file is empty
        if file.metadata().unwrap().len() != 0 {
            file.read_to_string(&mut contents)
                .expect("Failed to read session_id.toml file");
            session_id_toml = contents.parse::<Table>().expect("Failed to parse TOML");
        }

        // go to veeva url section
        let veeva_url_section = session_id_toml
            .entry(instance)
            .or_insert_with(|| toml::Value::Table(Table::new()))
            .as_table_mut()
            .unwrap();

        // update session_id
        veeva_url_section.insert(account.into(), toml::Value::String(session_id.to_string()));

        file = File::create("./session_id.toml").unwrap();
        file.write_all(toml::to_string(&session_id_toml).unwrap().as_bytes())
            .expect("Failed to write to session_id.toml file");

        session_id.to_string()
    } else {
        eprintln!("failed to generate session id");
        std::process::exit(1);
    }
}
