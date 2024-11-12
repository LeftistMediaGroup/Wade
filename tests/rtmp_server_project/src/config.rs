#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub mongo_uri: String,
    pub encryption_key: Option<[u8; 32]>,
}

impl Config {
    pub fn new(
        address: &str,
        port: u16,
        mongo_uri: &str,
        encryption_key: Option<[u8; 32]>
    ) -> Self {
        Self {
            address: address.to_string(),
            port,
            mongo_uri: mongo_uri.to_string(),
            encryption_key,
        }
    }
}
