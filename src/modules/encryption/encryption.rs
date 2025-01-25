use rand::rngs::OsRng;
use rand::RngCore;
use base64::{ engine::general_purpose, Engine as _ };
use std::path::PathBuf;
use home::home_dir;

pub async fn create_short_key() -> String {
    let key = generate_local_key().await;

    let pass = &key[..9];

    //println!("Pass: {:#?}", pass);

    pass.to_string()
}

pub fn does_admin_key_exist(local_path: &PathBuf) -> bool {
    let mut new_path = PathBuf::new();

    new_path.push(local_path);

    println!("LocalPath of Admin Key: {:#?}", new_path);

    if new_path.exists() {
        return true;
    } else {
        return false;
    }
}

pub async fn generate_local_key() -> String {
    let mut key = [0u8; 96];
    OsRng.fill_bytes(&mut key);
    base64::encode(&key);

    let encoded_key = general_purpose::STANDARD.encode(&key);

    //println!("key: {:#?}", encoded_key);

    let short_key = encoded_key.replace("/", "");

    //println!("Short: {:?}", short_key);

    short_key
}
