extern crate mongodb;
use mongodb::{ Client, options::ClientOptions };

use crate::modules::database::database::create_database;

pub async fn init_database_main() {
    println!("\nConnecting to localhost database");

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let database_names = client
        .list_database_names(None, None).await
        .expect("Failed to get databases");

    let mut found = false;

    for database in database_names.iter() {
        match database.as_str() {
            "Wade" => {
                found = true;
            }
            _ => {}
        }
    }

    if found {
        println!("Database Found");
    } else {
        println!("Database Not Found\nCreating new Database");

        create_database(&client).await;
    }
}
