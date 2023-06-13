use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use toml::Table;
use url::Url;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

pub async fn check_veeva_session_id(veeva_link: String) -> String {
    let veeva_url = Url::parse(veeva_link.as_str()).unwrap();

    let veeva_url = format!("{}://{}", veeva_url.scheme(), veeva_url.host_str().unwrap());

    // check if session_id.toml file exists and is empty
    if !std::path::Path::new("../session_id.toml").exists()
        || std::path::Path::new("../session_id.toml")
            .metadata()
            .unwrap()
            .len()
            == 0
    {
        return generate_new_session_id(veeva_url).await;
    }

    // read session_id.toml file
    let file = File::open("../session_id.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let session_id_table: Table = toml::from_str(contents.as_str()).unwrap();
    let session_id = session_id_table["veeva_url"][veeva_url.as_str()]
        .as_str()
        .unwrap();

    // check if session_id is valid
    let client = Client::new();
    let res = client
        .post(format!("{}/api/v23.1/keep-alive", veeva_url).as_str())
        .header(AUTHORIZATION, session_id.clone())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        // read response
        let body = res.text().await.unwrap();
        if body.contains("FAILURE") {
            generate_new_session_id(veeva_url).await
        } else {
            session_id.to_owned()
        }
    } else {
        eprintln!("Invalid session_id");
        std::process::exit(1);
    }
}

async fn generate_new_session_id(veeva_url: String) -> String {
    let client = Client::new();
    let res = client
        .post(format!("{}/api/v23.1/auth", veeva_url).as_str())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(ACCEPT, "application/json")
        .body("username=Bi_admin@indegene-cpp.com&password=Veeva@123456")
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        let value = res.json::<Value>().await.unwrap();
        let session_id = value["sessionId"].as_str().unwrap();
        // check if session_id.toml file exists
        let mut file: File;
        if !std::path::Path::new("../session_id.toml").exists() {
            // create session_id.toml file
            file = File::create("../session_id.toml").unwrap();
        } else {
            file = File::open("../session_id.toml").unwrap();
        }

        // parse session_id.toml file
        let mut contents = String::new();
        let mut session_id_toml = toml::value::Table::new();
        // check if file is empty
        if file.metadata().unwrap().len() != 0 {
            file.read_to_string(&mut contents)
                .expect("Failed to read session_id.toml file");
            session_id_toml = contents.parse::<Table>().expect("Failed to parse TOML");
        }

        // go to veeva url section
        let veeva_url_section = session_id_toml
            .entry("veeva_url")
            .or_insert_with(|| toml::Value::Table(toml::value::Table::new()))
            .as_table_mut()
            .unwrap();

        // update session_id
        veeva_url_section.insert(veeva_url, toml::Value::String(session_id.to_string()));

        file = File::create("../session_id.toml").unwrap();
        file.write_all(toml::to_string(&session_id_toml).unwrap().as_bytes())
            .expect("Failed to write to session_id.toml file");

        session_id.to_string()
    } else {
        eprintln!("Failed to generate new session_id");
        std::process::exit(1);
    }
}