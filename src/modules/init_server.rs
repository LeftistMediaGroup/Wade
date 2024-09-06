use axum::routing::get;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use socketioxide::SocketIo;

use crate::modules::init_socketio::init_socketio_main;

pub async fn init_server_main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::builder().build_layer();

    init_socketio_main(io);

    let app = axum::Router
        ::new()
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()).layer(layer));

    println!("Starting server");
    println!("Connect and point Front-End to localhost:5501 to continue Init");

    let listener = TcpListener::bind("0.0.0.0:5501").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
