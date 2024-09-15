use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use mongodb::{ Client, options::ClientOptions };
use bson::Document;
use serde_json::Value;
use bson::doc;
use crate::modules::manifest::manifest::Manifest;
use crate::modules::manifest::manifest::Init_Manifest;

use crate::modules::manifest::manifest::Manifest_In;
use crate::modules::database::database::Get_Manifest;
use crate::modules::encryption::encryption::generate_local_key;

pub fn init_socketio_main(io: SocketIo) {
    io.ns("/", |socket: SocketRef| {
        println!("socket connected: {}", socket.id);

        socket.on("join", |socket: SocketRef, Data::<String>(room)| async move {
            println!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let _ = socket.emit("messages", {});
        });

        socket.on("database_init", |s: SocketRef| async move {
            let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
            let client: Client = Client::with_options(client_options).unwrap();

            let collection: mongodb::Collection<bson::Document> = client
                .database("Wade")
                .collection("Init");

            let data_get_result = collection
                .find_one(
                    doc! {
                    "title": "Wade_Manifest"
                },
                    None
                ).await
                .expect("Reason");

            if data_get_result.is_none() {
                s.emit("database_init", "False");
            } else {
                s.emit("database_init", "True");
            }
        });

        socket.on("register_admin", |_socket: SocketRef, Data::<Value>(data)| async move {
            let data: Manifest_In = serde_json::from_str(&data.to_string()).unwrap();

            let manifest: Manifest = Init_Manifest(
                data.cause,
                data.organization,
                data.admin_name
            ).await;

            println!("Manifest: {:#?}", manifest);
        });

        socket.on("Encrypt", |_socket: SocketRef| async move {
            println!("Encrypt:");
            let _local_key = generate_local_key();
        });
    });
}

/*
socket.on("message", |_socket: SocketRef, Data::<String>(data)| async move {
println!("Received message: {:?}", data);
});
*/
