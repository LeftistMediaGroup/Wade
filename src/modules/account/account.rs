use crate::modules::{crypto::crypto::*, database::database::*, encryption::encryption::*};
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{options::ClientOptions, Client, Collection};
use rand::prelude::*;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};

pub async fn get_account(name: &String) -> Option<User> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&String::from("Wade"));

    let coll: Collection<User> = db.collection(&String::from("accounts"));
    let result: Option<User> = match coll.find_one(doc! { "username": name }, None).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(_) => None, // Ignore the error and return None
    };
    result
}

pub async fn get_accounts() -> Vec<Account> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&String::from("Wade"));

    let coll: Collection<Account> = db.collection(&String::from("accounts"));

    let mut users = Vec::new();

    let mut cursor = coll.find_one(None, None).await.unwrap();
    while let Some(doc) = cursor.try_next().await.unwrap() {
        let account = Account {
            username: doc.clone().username,
            is_admin: doc.clone().is_admin,
        };
        println!("{:#?}", account);
        users.push(account);
    }

    users
}

pub async fn create_admin(username: String, password: String, is_admin: bool) {
    let encrypt_test = encrypt(&password, &String::from("ecrypt_test"))
        .await
        .unwrap();

    let user = User {
        username: username,
        is_admin: is_admin,
        encrypt_test: encrypt_test,
    };

    put_data("accounts", user).await;
}

pub async fn generate_user() -> TempUser {
    let password = create_short_key().await;

    let rng = RNG::try_from(&Language::Demonic).unwrap();

    let first = rng.generate_name();
    let last = rng.generate_name();

    let mut rand = thread_rng();

    let num = rand.gen_range(0..100).to_string();
    let username = first + "-" + &last + "-" + &num;

    TempUser {
        username: username,
        is_admin: false,
    }
}

pub async fn create_user(username: String, password: String) -> User {
    let encrypt_test = encrypt(&password, &String::from("ecrypt_test"))
        .await
        .unwrap();

    let user = User {
        username: username,
        is_admin: false,
        encrypt_test: encrypt_test,
    };

    put_data("accounts", user.clone()).await;

    user
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TempUser {
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub is_admin: bool,
    pub encrypt_test: String,
}
