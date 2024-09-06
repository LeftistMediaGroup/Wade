extern crate mongodb;
use mongodb::{ Client, options::ClientOptions, Collection };
use bson::doc;
use bson::Document;
use std::error;
use crate::modules::manifest::manifest::Manifest;
use crate::modules::manifest::manifest::Init_Manifest;

pub async fn Get_Data(document_id: String) -> Vec<i32> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let data_get_result: Result<
        std::option::Option<bson::Document>,
        mongodb::error::Error
    > = collection.find_one(
        doc! {
                "title": "Wade_Manifest"
            },
        None
    ).await;

    match data_get_result {
        Ok(file) => {
            match file {
                None => {
                    println!("No Manifest Found in database\nCreating Manifest now");

                    let manifest: Manifest = Init_Manifest().await;
                }
                Some(file) => {
                    println!("FOUND FILE");
                    file;
                }
            }
        }
        Err(_) => todo!(),
    }

    let final_data = vec![0, 1];

    final_data
}
