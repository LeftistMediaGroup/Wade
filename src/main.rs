mod modules;
use modules::{ init_server::init_server_main, init_database::init_database_main };

use dotenv::dotenv;

async fn init_modules() {
    init_database_main().await;

    init_server_main().await.expect("REASON")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("\n\nWelcome to Wade!\n");

    init_modules().await;
}
