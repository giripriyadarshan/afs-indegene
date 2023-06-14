use std::sync::Arc;
use toml::Table;
use url::Url;

use reqwest::{
    blocking::multipart::Form,
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};

pub async fn upload_to_vault(
    file_name: String,
    vault_url: Arc<String>,
    zip_file_path: String,
    key_message_id: Arc<Table>,
    session_id: Arc<String>,
) {
    let doc_id = key_message_id
        .get(file_name.as_str())
        .unwrap()
        .as_str()
        .unwrap();
    let vault_url = Url::parse(vault_url.as_str()).unwrap();
    let vault_url = format!("{}://{}", vault_url.scheme(), vault_url.host_str().unwrap());

    let client = Client::new();
    let res = client
        .post(format!("{}/api/v19.1/objects/documents/{}/lock", vault_url, doc_id).as_str())
        .header(AUTHORIZATION, session_id.as_str())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        println!("{} locked successfully", file_name.clone());

        let form = Form::new().file("file", zip_file_path.as_str()).unwrap();
        let boundary = form.boundary();

        println!("Uploading {}...", file_name.clone());
        let client = reqwest::blocking::Client::new();
        let upload_res = client
            .post(format!("{}/api/v19.1/objects/documents/{}", vault_url, doc_id).as_str())
            .header(AUTHORIZATION, session_id.as_str())
            .header(
                CONTENT_TYPE,
                format!("multipart/form-data; boundary={}; charset=UTF-8", boundary),
            )
            .multipart(form)
            .send()
            // .await
            .unwrap();

        if upload_res.status().is_success() {
            println!("{:?}", upload_res.text().unwrap());
        } else {
            println!("{} failed to upload", file_name);
        }
    }
}
