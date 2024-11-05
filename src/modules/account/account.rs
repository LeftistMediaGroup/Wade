use crate::modules::database::database::get_data;
use bson::Document;

pub async fn does_account_exist(name: &String) -> Option<Document> {
    let data = get_data(&String::from("account"), &String::from(name)).await;

    data
}

pub fn create_admin(username: String) {}

pub fn create_user(username: String) {}
