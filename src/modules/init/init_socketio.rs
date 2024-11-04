use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use bson::Document;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use bson::doc;
use rnglib::{ RNG, Language };

use crate::modules::{ manifest::manifest::*, encryption::encryption::*, crypto::crypto::* };

use super::super::system::system::*;

pub fn init_socketio_main(io: SocketIo) {
    io.ns("/", |socket: SocketRef| {
        println!("\nsocket connected: {}\n", socket.id);

        socket.on("join", |socket: SocketRef, Data::<String>(room)| async move {
            println!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let _ = socket.emit("messages", {});
        });

        socket.on("wade_init", |s: SocketRef| async move {
            let result = does_manifest_exist().await;
            if result {
                s.emit("wade_init", true);
            } else {
                s.emit("wade_init", false);
            }
        });

        socket.on("Encrypt", |s: SocketRef| async move {
            let admin_key = create_short_key().await;

            s.emit("encrypt_admin", admin_key);
        });

        socket.on("manifest_init", |s: SocketRef, Data::<Value>(data)| async move {
            let data: Admin_data = serde_json::from_str(&data.to_string()).unwrap();

            println!("Data: {:#?}", data);

            println!("DATA: {:?}", data.admin_pass);

            let encrypted = encrypt(&data.admin_pass, &data.cause).await.unwrap();

            println!("Encrypted: {:?}", encrypted);

            let decypted = decrypt(&data.admin_pass, &encrypted).await.unwrap();

            println!("Decrypted: {:?}", decypted);

            /*let _manifest: Manifest = init_manifest(
                data.cause,
                data.organization,
                data.admin_name
            ).await;

            //let result = encrypt_doc(doc! { "test": "test" }).await;

            //println!("Result: {:#?}", result);
            */
        });

        socket.on("create_user", |_s: SocketRef, Data::<Value>(data)| async move {
            println!("Data: {:#?}", data);
        })
    });
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin_data {
    pub cause: String,
    pub organization: String,
    pub admin_name: String,
    pub admin_pass: String,
}
