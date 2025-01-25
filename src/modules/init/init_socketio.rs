use bson::doc;
use bson::Document;
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};

use crate::modules::{
    account::account::*, crypto::crypto::*, encryption::encryption::*, manifest::manifest::*,
};
use std::convert::TryInto;
use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
pub fn init_socketio_main(io: SocketIo) {
    io.ns("/", |socket: SocketRef| {
        println!("\nsocket connected: {}\n", socket.id);

        socket.on(
            "join",
            |socket: SocketRef, Data::<String>(room)| async move {
                println!("Received join: {:?}", room);
                let _ = socket.leave_all();
                let _ = socket.join(room.clone());
                let _ = socket.emit("messages", {});
            },
        );

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

        socket.on(
            "manifest_init",
            |s: SocketRef, Data::<Value>(data)| async move {
                let data: Admin_data = serde_json::from_str(&data.to_string()).unwrap();

                let manifest = init_manifest(
                    encrypt(&data.admin_pass, &data.cause).await.unwrap(),
                    encrypt(&data.admin_pass, &data.organization).await.unwrap(),
                    encrypt(&data.admin_pass, &data.admin_name).await.unwrap(),
                )
                .await;

                println!("Manifest: {:?}", manifest);

                create_admin(data.admin_name, data.admin_pass, true).await;
            },
        );

        socket.on("log_in", |s: SocketRef, Data::<Value>(data)| async move {
            let data: Login_data = serde_json::from_str(&data.to_string()).unwrap();

            let user = get_account(&data.username).await;

            match user {
                None => {
                    s.emit("log_in", false);
                }
                _ => ({
                    let decrypted =
                        decrypt(&data.password, &user.clone().unwrap().encrypt_test).await;
                    match decrypted {
                        Ok(_) => {
                            let data_out = Data_Out {
                                is_logged_in: true,
                                username: data.username.to_string(),
                                is_admin: user.clone().unwrap().is_admin,
                            };
                            s.emit("log_in", data_out)
                        }
                        Err(_) => {
                            let data_out = Data_Out {
                                is_logged_in: false,
                                username: "".to_string(),
                                is_admin: false,
                            };
                            s.emit("log_in", data_out)
                        }
                    }
                })
                .expect("REASON"),
            }
        });

        socket.on("gen-user", |s: SocketRef| async move {
            let user = generate_user().await;

            let short_pass = create_short_key().await;

            let user_out = User_Out {
                username: user.username,
                short_pass: short_pass,
            };
            s.emit("user_details", user_out);
        });

        socket.on(
            "submit_user",
            |s: SocketRef, Data::<Value>(data)| async move {
                let data: UserData = serde_json::from_str(&data.to_string()).unwrap();

                let user = create_user(data.username, data.password).await;

                println!("Created: {:?}", user);
            },
        );

        socket.on("get_accounts", |s: SocketRef| async move {
            let accounts: Vec<Account> = get_accounts().await;

            println!("Accounts: {:?}", accounts);

            //s.emit("accounts", accounts_out);
        })
    });
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User_Out {
    pub username: String,
    pub short_pass: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data_Out {
    pub is_logged_in: bool,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin_data {
    pub cause: String,
    pub organization: String,
    pub admin_name: String,
    pub admin_pass: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Login_data {
    pub username: String,
    pub password: String,
}
