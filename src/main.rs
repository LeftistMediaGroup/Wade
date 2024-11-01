mod modules;
use modules::init::{ init_server::init_server_main, init_database::init_database_main };
use std::process::Command;

use dotenv::dotenv;

async fn init_modules() {
    //init_database_main().await;

    init_server_main().await.expect("REASON")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("\n\nWelcome to Wade!\n");

    launch_frontend();

    init_modules().await;
}

pub fn launch_frontend() {
    Command::new("npm").args(vec!["run", "dev", "--prefix", "lmg"]).spawn().unwrap();
}
