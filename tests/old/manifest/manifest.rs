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
use mongodb::results::InsertOneResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest_In {
    pub cause: String,
    pub organization: String,
    pub admin_name: String,
}

pub async fn Init_Manifest(
    cause: String,
    organization: String,
    admin_name: String,
) -> Manifest {
    let init_time = {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        datetime.format("%y%m%d_%X").to_string()
    };

    let manifest: Manifest = New_Manifest(cause, organization, admin_name).await;

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

    //insert_encrypted_doc(new_doc).await;

    manifest
}

pub async fn New_Manifest(
    cause: String,
    organization: String,
    admin_name: String,
) -> Manifest {
    let manifest: Manifest = Manifest::New(cause, organization, admin_name).await;
    manifest
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    name: String,
    causes: Vec<Cause>,
}

impl Manifest {
    pub async fn New(
        cause: String,
        organization: String,
        admin_name: String,
    ) -> Manifest {
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
    create_time: String,

    orginizations: Vec<Organization>,
}

impl Cause {
    pub fn New_Cause(
        cause: String,
        organization: String,
        admin_name: String,
    ) -> Cause {
        let cause: Cause = Cause {
            cause_name: cause,
            create_time: Self::Cause_Init_Time(),
            orginizations: vec![
                Organization::New_Organization(organization, admin_name)
            ],
        };

        cause
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
    pub fn New_Organization(
        organization: String,
        admin_name: String,
    ) -> Organization {
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
