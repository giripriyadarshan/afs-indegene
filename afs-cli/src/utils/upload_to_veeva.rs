use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use toml::Table;

use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};

pub async fn upload_to_vault(file_name: String, vault_url: String, key_message_id: Arc<Table>) {}
