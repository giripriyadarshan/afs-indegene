use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
   pub vault: Vault,
   pub files: Files
}

#[derive(Deserialize, Debug)]
pub struct Files {
   pub key_messages_file: String,
   pub local_shared_folder: String,
   pub pdf_script_file: String,
}

#[derive(Deserialize, Debug)]
pub struct Vault {
   pub link: String,
   pub vault_shared_folder: String,
}