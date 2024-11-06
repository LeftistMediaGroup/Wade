use mongodb::{ Client, options::ClientOptions };
use bson::{ doc, Document };
use std::io::Error;
use serde::Serialize;

pub async fn create_database() {
    let client_options: ClientOptions = ClientOptions::parse(
        "mongodb://localhost:27017"
    ).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db_name = String::from("Wade");
    let coll_name = String::from("Init");

    create_collection(&coll_name).await;
}

pub async fn create_collection(coll_name: &str) {
    let client_options: ClientOptions = ClientOptions::parse(
        "mongodb://localhost:27017"
    ).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&String::from("Wade"));
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

pub async fn put_data<T>(coll_name: &str, doc: T) where T: Serialize {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("Wade");
    let coll = db.collection::<Document>(coll_name);

    // Convert the document to a BSON Document
    let bson_doc = mongodb::bson::to_document(&doc).unwrap();

    coll.insert_one(bson_doc, None).await.unwrap();
}
