use std::io;
use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::io::BufRead;
use bson::Document;
use bson::doc;

use serde::{ Deserialize, Serialize };
use mongodb::bson::{ Bson, oid::ObjectId };

use mongodb::{ Client, options::ClientOptions, Collection };
use crate::modules::database::database;
use mongodb::results::InsertOneResult;

pub async fn Get_Manifest() -> Vec<i32> {
    let manifest: Vec<i32> = database::Get_Data(String::from("Manifest")).await;
    manifest
}

pub async fn Init_Manifest() -> Manifest {
    let init_time = {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        datetime.format("%y/%m%d_%X").to_string()
    };

    let manifest: Manifest = New_Manifest().await;

    let bson_manifest = bson::to_bson(&manifest).expect("BSON ERROR");

    let new_doc =
        doc! {
        "title": "Wade_Manifest",
        "init_time": init_time,
        "details": bson_manifest,
     };

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();

    let collection: mongodb::Collection<bson::Document> = client
        .database("Wade")
        .collection("Init");

    let doc_insert_result: Result<Result<InsertOneResult, _>, mongodb::error::Error> = Ok(
        collection.insert_one(new_doc.clone(), None).await
    );

    println!("Inserted Manifest: {:#?}", doc_insert_result);

    manifest
}

pub async fn New_Manifest() -> Manifest {
    let manifest: Manifest = Manifest::New().await;
    manifest
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    name: String,
    causes: Vec<Cause>,
}

impl Manifest {
    pub async fn New() -> Manifest {
        let manifest: Manifest = Manifest {
            name: String::from("Wade"),
            causes: vec![Cause::New_Cause()],
        };

        manifest
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cause {
    cause_name: String,
    create_time: String,
    mongodb_url: String,
    orginizations: Vec<Organization>,
}

impl Cause {
    pub fn New_Cause() -> Cause {
        let cause: Cause = Cause {
            cause_name: Self::Cause_Name(),
            create_time: Self::Cause_Init_Time(),
            mongodb_url: String::from("mongodb://localhost:27017"),
            orginizations: vec![Organization::New_Organization()],
        };

        cause
    }

    pub fn Cause_Name() -> String {
        let stdin = io::stdin();

        println!("Enter New Cause\n");

        let locked = stdin.lock();
        let input: Vec<String> = locked
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        println!("Cause: {}", input[0]);

        input[0].clone()
    }

    pub fn Cause_Init_Time() -> String {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        datetime.format("%y/%m%d_%X").to_string()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    orginaization_name: String,
    orginization_admins: Vec<Admin>,
}

impl Organization {
    pub fn New_Organization() -> Organization {
        let organization: Organization = Organization {
            orginaization_name: Self::Organization_Name(),
            orginization_admins: vec![Admin::New_Admin()],
        };

        organization
    }

    pub fn Organization_Name() -> String {
        let stdin = io::stdin();

        println!("Enter New Organization Name\n");

        let locked = stdin.lock();
        let input: Vec<String> = locked
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        println!("Organization: {}", input[0]);

        input[0].clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin {
    admin_name: String,
    admin_pass: String,
}

impl Admin {
    pub fn New_Admin() -> Admin {
        let admin: Admin = Admin {
            admin_name: Self::Admin_Name(),
            admin_pass: Self::Admin_Pass(),
        };

        admin
    }

    pub fn Admin_Name() -> String {
        let stdin = io::stdin();

        println!("Enter Admin Name\n");

        let locked = stdin.lock();
        let input: Vec<String> = locked
            .lines()
            .filter_map(|line| line.ok())
            .collect();
        println!("Admin: {}", input[0].clone());

        input[0].clone()
    }

    pub fn Admin_Pass() -> String {
        let finalpass = loop {
            let stdin = io::stdin();

            println!("Enter Admin Password\n");

            let locked = stdin.lock();
            let input1: Vec<String> = locked
                .lines()
                .filter_map(|line| line.ok())
                .collect();

            println!("Reenter Admin Password\n");

            let locked = stdin.lock();
            let input2: Vec<String> = locked
                .lines()
                .filter_map(|line| line.ok())
                .collect();

            if input1[0].clone() == input2[0].clone() {
                println!("Passwords Matched");

                break input1[0].clone();
            } else {
                println!("Passwords do not match, Try Again");
            }
        };

        finalpass
    }
}
