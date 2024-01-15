use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Aws {
    pub region: String,
    pub credentials_file: String,
    pub bucket_name: String,
    pub object_key: String,
}
