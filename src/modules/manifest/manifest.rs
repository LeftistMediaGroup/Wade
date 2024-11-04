use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use bson::Document;
use bson::doc;
use mongodb::{ Client, options::ClientOptions, Collection };

use serde::{ Deserialize, Serialize };

pub async fn does_manifest_exist() -> bool {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let result = match collection.find_one(doc! { "title": "Wade_Manifest" }, None).await {
        Ok(Some(data)) => { true }
        Err(err) => { false }
        Ok(None) => { false }
    };

    result
}

pub async fn init_manifest(cause: String, organization: String, admin_name: String) -> Manifest {
    let manifest: Manifest = new_manifest(cause, organization, admin_name).await;

    let bson_manifest = bson::to_bson(&manifest).expect("BSON ERROR");

    let new_doc = doc! {
        "title": "Wade_Manifest",
        "details": bson_manifest,
     };

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    collection.insert_one(new_doc.clone(), None).await;

    manifest
}

pub async fn get_manifest() -> bool {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let manifest = match collection.find_one(doc! { "title": "Wade_manifest" }, None).await {
        Ok(Some(data)) => { true }
        Err(err) => { false }
        Ok(None) => { false }
    };
    manifest
}

fn handle_no_document_found() {
    println!("Not found")
}

pub async fn new_manifest(cause: String, organization: String, admin_name: String) -> Manifest {
    let manifest: Manifest = Manifest::New(cause, organization, admin_name).await;
    manifest
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest_In {
    pub cause: String,
    pub organization: String,
    pub admin_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    name: String,
    causes: Vec<Cause>,
}

impl Manifest {
    pub async fn New(cause: String, organization: String, admin_name: String) -> Manifest {
        let manifest: Manifest = Manifest {
            name: String::from("Wade"),
            causes: vec![Cause::New_Cause(cause, organization, admin_name)],
        };

        manifest
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cause {
    cause_name: String,

    orginizations: Vec<Organization>,
}

impl Cause {
    pub fn New_Cause(cause: String, organization: String, admin_name: String) -> Cause {
        let cause: Cause = Cause {
            cause_name: cause,
            orginizations: vec![Organization::New_Organization(organization, admin_name)],
        };

        cause
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    orginaization_name: String,
    orginization_admins: Vec<Admin>,
}

impl Organization {
    pub fn New_Organization(organization: String, admin_name: String) -> Organization {
        let organization: Organization = Organization {
            orginaization_name: organization,
            orginization_admins: vec![Admin::New_Admin(admin_name)],
        };

        organization
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin {
    admin_name: String,
}

impl Admin {
    pub fn New_Admin(admin_name: String) -> Admin {
        let admin: Admin = Admin {
            admin_name: admin_name,
        };

        admin
    }
}
