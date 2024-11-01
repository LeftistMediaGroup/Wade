use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use socketioxide::SocketIo;
use http::Method;

use super::init_socketio::init_socketio_main;

pub async fn init_server_main() -> Result<(), Box<dyn std::error::Error>> {
    let (socketio_layer, io) = SocketIo::builder().build_layer();

    init_socketio_main(io);

    let origins = ["http://localhost:5173".parse().unwrap()];

    let layer_cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST]);

    let app = axum::Router
        ::new()
        .layer(ServiceBuilder::new().layer(layer_cors).layer(socketio_layer));

    println!("Starting server");

    let listener = TcpListener::bind("0.0.0.0:5501").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
