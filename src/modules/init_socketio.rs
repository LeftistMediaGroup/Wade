use socketioxide::{ extract::{ Data, SocketRef }, SocketIo };
use mongodb::{ Client, options::ClientOptions };
use bson::Document;

pub fn init_socketio_main(io: SocketIo) {
    io.ns("/", |socket: SocketRef| {
        println!("socket connected: {}", socket.id);

        socket.on("join", |socket: SocketRef, Data::<String>(room)| async move {
            println!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let _ = socket.emit("messages", {});
        });

        socket.on("is_init", |socket: SocketRef| async move {
            println!("Recieved Front-End Init connection!");

            socket.emit("is_init", {}).ok();
        });

        socket.on("init_manifest", |socket: SocketRef, Data::<String>(data)| async move {
            println!("Data: {:#?}", data)
        });
    });
}

/*
socket.on("message", |_socket: SocketRef, Data::<String>(data)| async move {
println!("Received message: {:?}", data);
});
*/
