use mongodb::{ Client, options::ClientOptions };
use bson::{ doc, Document };
use std::io::Error;

pub async fn create_database(client: &Client) {
    let db_name = String::from("Wade");
    let coll_name = String::from("Init");

    create_collection(&client, &db_name, &coll_name).await;
}

pub async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap();

    println!("Database Created");
}

pub async fn get_data(coll_name: &str, title: &str) -> Option<Document> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&String::from("Wade"));

    let coll = db.collection(coll_name);
    let result: Option<Document> = match coll.find_one(doc! { "title": title }, None).await {
        Ok(Some(manifest)) => manifest,
        Ok(None) => None,
        Err(_) => None, // Ignore the error and return None
    };
    result
}
