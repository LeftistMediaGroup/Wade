use mongodb::error::Error;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::File;
use std::io::{ Write, Read };
use std::{ env, option };
use std::path::PathBuf;
use mongodb::{ Client, options::ClientOptions };
use mongodb::bson::{ doc, Binary, spec::BinarySubtype, Document };

use base64::{ engine::general_purpose, Engine as _ };

pub async fn generate_local_key() -> String {
    let mut key = [0u8; 96];
    OsRng.fill_bytes(&mut key);
    base64::encode(&key);

    let mut encoded_key = general_purpose::STANDARD.encode(&key);

    encoded_key.truncate(10);

    println!("Local key: {:#?}", encoded_key);

    encoded_key
}

pub fn write_key_to_file(key: &str, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(key.as_bytes())?;

    println!("ADMIN CREDS SAVED TO: {:?}", file_path);

    Ok(())
}

pub fn read_key_from_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut key = String::new();
    file.read_to_string(&mut key)?;
    Ok(key)
}

pub fn get_home_directory() -> PathBuf {
    env::home_dir().expect("Failed to get home directory")
}

pub async fn insert_document(
    collection: &mongodb::Collection<Document>,
    document: bson::Document
) -> mongodb::error::Result<()> {
    match collection.insert_one(document, None).await {
        Ok(insert_result) => println!("Document inserted: {:?}", insert_result),
        Err(e) => eprintln!("Error inserting document: {:?}", e),
    }
    Ok(())
}

pub async fn insert_encrypted_doc(document: Document) -> mongodb::error::Result<()> {
    let home_dir = get_home_directory();
    let key_file_path = home_dir.join("local_key.txt");
    let key_file_path_str = key_file_path.to_str().expect("Failed to convert path to string");
    let local_key = generate_local_key().await;

    write_key_to_file(&local_key, key_file_path_str).expect("Failed to write key to file");
    let loaded_key = read_key_from_file(key_file_path_str).expect("Failed to read key from file");

    let kms_providers =
        doc! {
        "local": {
            "key": Binary {
                subtype: BinarySubtype::Generic,
                bytes: base64::decode(&loaded_key).unwrap(),
            }
        }
    };

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await;
    let encrypted_client = Client::with_options(client_options?);

    // Perform encrypted read/write operations
    let collection = encrypted_client?.database("Wade").collection("Init");

    // Insert a document
    insert_document(&collection, document).await?;

    Ok(())
}
