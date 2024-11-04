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

pub async fn Get_Manifest() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let manifest = match collection.find_one(doc! { "title": "Wade_manifest" }, None).await {
        Ok(Some(manifest)) => {
            println!("Found :\n{:#?}", manifest);
        }
        Ok(None) => {
            handle_no_document_found();
        }
        Err(err) => { handle_no_document_found() }
    };
}

fn handle_no_document_found() {
    println!("Not found")
}
