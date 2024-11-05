use crate::modules::{ database::database::*, crypto::crypto::* };

use bson::{ doc, Document };
use serde::{ Deserialize, Serialize };

pub async fn does_account_exist(name: &String) -> Option<Document> {
    let data = get_data(&String::from("accounts"), &String::from(name)).await;

    data
}

pub async fn create_user(username: String, password: String, is_admin: &String) {
    let encrypt_test = encrypt(&password, &String::from("ecrypt_test")).await.unwrap();

    let doc =
        doc! {
        "title": username,
        "is_admin": is_admin,
        "encrpt_test": encrypt_test
    };

    put_data("accounts", doc.clone()).await;
}
