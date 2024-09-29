use axum::extract::path;
use bson::{ raw::Error, Document };
use mongodb::Collection;
use serde::de::value::BoolDeserializer;
use tokio::sync::mpsc::WeakSender;
use std::fs;
use std::path::{ Path, PathBuf };
use rand::rngs::OsRng;
use rand::RngCore;
use base64::{ engine::general_purpose, Engine as _ };
use std::fs::File;
use std::io::{ Write, Read };

use home::home_dir;

pub async fn does_key_exist() -> Key_Data {
    let mut path = PathBuf::new();
    let home_dir = home_dir().unwrap();

    path.push(&home_dir);
    path.push("Wade");

    let mut admin_key_path = PathBuf::new();
    admin_key_path.push(&path);
    admin_key_path.push("Admin_key.txt");

    println!("Path: {:#?}", path);

    if path.exists() {
        println!("Exists");
    } else {
        println!("Does Not Exist, ceating now");

        create_wade_dir();
    }

    if admin_key_path.exists() {
        let data: Key_Data = Key_Data {
            created: "True".to_string(),
            pass: Default::default(),
        };

        return data;
    } else {
        println!("No Admin Key, Creating Now");

        let admin_key = create_admin_key().await;

        let data: Key_Data = Key_Data {
            created: "False".to_string(),
            pass: Some(admin_key),
        };

        return data;
    }
}

pub async fn create_admin_key() -> String {
    let mut path = PathBuf::new();
    let home_dir = home_dir().unwrap();

    path.push(&home_dir);
    path.push("Wade");

    let mut admin_key_path = PathBuf::new();
    admin_key_path.push(&path);
    admin_key_path.push("Admin_key.txt");

    let key = generate_local_key().await;

    let pass = &key[..7];
    let end_key = &key[7..];

    println!("Pass: {:#?}", pass);
    println!("Save: {:#?}", end_key);

    //write_key_to_file(end_key.clone(), &admin_key_path).await;

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

pub fn read_key_from_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut key = String::new();
    file.read_to_string(&mut key)?;
    Ok(key)
}

pub fn create_wade_dir() {
    let mut path = PathBuf::new();
    let home_dir = home_dir().unwrap();

    path.push(&home_dir);
    path.push("Wade");

    std::fs::create_dir(path);
}

pub async fn write_key_to_file(key: &str, file_path: &PathBuf) {
    let mut file = File::create(file_path).unwrap();

    file.write_all(key.as_bytes());

    println!("CREDS SAVED TO: {:?}", file_path);
}

pub async fn generate_local_key() -> String {
    let mut key = [0u8; 96];
    OsRng.fill_bytes(&mut key);
    base64::encode(&key);

    let encoded_key = general_purpose::STANDARD.encode(&key);

    println!("key: {:#?}", encoded_key);

    encoded_key
}

#[derive(Default)]
pub struct Key_Data {
    pub created: String,
    pub pass: Option<String>,
}
/*





*/
pub struct DatabaseManifest {
    pub collection_manifest: CollectionManifest,
    pub users_manifest: UsersManifest,
    pub passwords_manifest: PasswordManifest,
}

pub struct CollectionManifest {
    pub collection_name: String,
    pub key_location: String,
}

pub enum PasswordManifest {
    AdminPass(String),
    UserPass(String),
}

pub enum UsersManifest {
    AdminUser(Admin),
    User(User),
}

pub struct Admin {
    admin_name: String,
}

impl Admin {
    pub fn new_admin(admin_name: String) -> Admin {
        let admin = Admin {
            admin_name,
        };

        admin
    }
}

pub struct User {
    pub username: String,
}

impl User {
    pub fn new_user(username: String) -> User {
        let user = User {
            username,
        };

        user
    }
}

/* 

    Wade
        init_data

        database_manifest
            password manifest
                AdminPass
                    admin = passlocation

                    admin2 = passlocation
                
                UserPass
                    user1 = passlocation

                    user2 = passlocation
            
            UsersManifest

            CollectionManifest

    video_chart

    sophia_data

    forms

    news 
        
    user1
        profile
        
        data

        chat
    user2
        profile
            
        data

        chat
 */

/*
 USB
    Vault (vera)
        Admin_key

        Admin1_key
        Admin2_key
        
        User1_key
        User2_key
 */
