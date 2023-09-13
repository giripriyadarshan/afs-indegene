use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;
use toml::Table;
use url::Url;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};


pub async fn get_session_id(instance: impl Into<String>) -> String {
    // read from sessions.toml file
    // check if the session id is valid (check_session_id())
    // if valid, return the session id
    // if not valid, renew the session id (renew_session_id())
    // return the session id

    let instance = instance.into();
    // check if session_id.toml file exists and is empty
    if !std::path::Path::new("../session_id.toml").exists()
        || std::path::Path::new("../session_id.toml")
        .metadata()
        .unwrap()
        .len()
        == 0
    {
        return renew_session_id( instance).await;
    }

    let file = File::open("../session_id.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let session_id_table: Table = toml::from_str(contents.as_str()).unwrap();
    let session_id = session_id_table["veeva_url"][instance.as_str()]
        .as_str()
        .unwrap();

    // check if session_id is valid
    let client = Client::new();
    let res = client
        .post(format!("{}/api/v23.1/keep-alive", instance).as_str())
        .header(AUTHORIZATION, session_id.clone())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        // read response
        let body = res.text().await.unwrap();
        if body.contains("FAILURE") {
            return renew_session_id(instance).await
        } else {
            return session_id.to_owned();
        }
    } else {
        eprintln!("Error: {}", res.status());
        std::process::exit(1);
    }
}

async fn renew_session_id(instance: impl Into<String>) -> String {
    // get password from zoho_auth.rs
    // decode the password using secret key from $ZOHO_SECRET_KEY
    // renew the session id
    // write to sessions.toml file
    // return the session id
    todo!()
}
