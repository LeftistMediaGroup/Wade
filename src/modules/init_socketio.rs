use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use mongodb::{ Client, options::ClientOptions };
use bson::Document;

use serde_json::Value;
use serde_json::json;
use bson::doc;
use crate::modules::{
    encryption::local_encryption::{ does_key_exist },
    manifest::manifest::Manifest,
};
use crate::modules::manifest::manifest::Init_Manifest;

use crate::modules::manifest::manifest::Manifest_In;
use crate::modules::database::database::Get_Manifest;
use crate::modules::encryption::encryption::generate_short_key;

use rnglib::{ RNG, Language };

pub fn init_socketio_main(io: SocketIo) {
    io.ns("/", |socket: SocketRef| {
        println!("\nsocket connected: {}\n", socket.id);

        socket.on("join", |socket: SocketRef, Data::<String>(room)| async move {
            println!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let _ = socket.emit("messages", {});
        });

        socket.on("database_init", |s: SocketRef| async move {
            let data = does_key_exist().await;

            let pass = data.pass;
            let data_get_result = data.created;

            println!("Data: {:#?}", data_get_result);

            if data_get_result == "False".to_string() {
                s.emit("database_init", "False");
                s.emit("admin_pass", pass);
            } else {
                s.emit("database_init", "True");
            }
        });

        socket.on("manifest_init", |s: SocketRef, Data::<Value>(data)| async move {
            let data: Manifest_In = serde_json::from_str(&data.to_string()).unwrap();

            let _manifest: Manifest = Init_Manifest(
                data.cause,
                data.organization,
                data.admin_name,
                data.admin_pass
            ).await;

            //let result = encrypt_doc(doc! { "test": "test" }).await;

            //println!("Result: {:#?}", result);
        });

        socket.on("new_user", |s: SocketRef| async move {
            println!("Encrypt:");
            let local_key = generate_short_key().await;

            let rng = RNG::try_from(&Language::Demonic).unwrap();

            let first_name = rng.generate_name();
            let last_name = rng.generate_name();

            let data_out =
                doc! {
                "username": first_name + "-" + &last_name,
                "local_key": local_key
            };

            s.emit("user_details", data_out);
        });

        socket.on("create_user", |_s: SocketRef, Data::<Value>(data)| async move {
            println!("Data: {:#?}", data);
        })
    });
}
