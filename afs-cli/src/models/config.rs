use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vault: Vault,
    pub files: Files,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub key_messages_file: String,
}

#[derive(Deserialize, Debug)]
pub struct Vault {
    pub link: String,
}
