use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RevisionConfig {
    pub revision: Revision,
}

#[derive(Deserialize, Debug)]
pub struct Revision {
    pub revision_number: usize,
}
