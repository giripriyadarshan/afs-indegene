use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vault: Vault,
}

#[derive(Deserialize, Debug)]
pub struct Vault {
    pub link: String,
    pub binder_id: String,
}
