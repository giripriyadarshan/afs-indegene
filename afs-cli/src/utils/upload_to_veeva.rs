use std::fs::File;
use std::io::Read;

use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE}};


pub async fn upload_to_vault(file_name: String, vault_url: String) {
    
}