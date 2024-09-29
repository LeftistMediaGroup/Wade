extern crate mongodb;
use mongodb::{ Client, options::ClientOptions, Collection };
use bson::doc;
use bson::Document;
use std::error;
use bson::Bson;
use serde::{ Deserialize, Serialize };

use mongodb::{ bson::{ Binary } };
use base64;
use rand::Rng;

use crate::modules::manifest::manifest::Manifest;
use crate::modules::manifest::manifest::Init_Manifest;

pub async fn Get_Manifest(url: String) -> Result<Document, mongodb::error::Error> {
    let client_options = ClientOptions::parse(url).await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let data_get_result = collection
        .find_one(doc! {
                "title": "Wade_manifest"
            }, None).await
        .expect("Reason");

    println!("Manifest: {:#?}", data_get_result);

    Ok(data_get_result.expect("REASON"))
}
