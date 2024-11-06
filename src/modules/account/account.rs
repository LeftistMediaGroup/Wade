use crate::modules::{ database::database::*, crypto::crypto::* };
use serde::{ Deserialize, Serialize };
use bson::{ doc, Document };
use mongodb::{ Client, options::ClientOptions, Collection };

pub async fn get_account(name: &String) -> Option<User> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&String::from("Wade"));

    let coll: Collection<User> = db.collection(&String::from("accounts"));
    let result: Option<User> = match coll.find_one(doc! { "username": name }, None).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => { None }
        Err(_) => None, // Ignore the error and return None
    };
    result
}

pub async fn create_user(username: String, password: String, is_admin: String) {
    let encrypt_test = encrypt(&password, &String::from("ecrypt_test")).await.unwrap();

    let user = User {
        username: username,
        is_admin: is_admin,
        encrypt_test: encrypt_test,
    };

    put_data("accounts", user).await;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub is_admin: String,
    pub encrypt_test: String,
}
