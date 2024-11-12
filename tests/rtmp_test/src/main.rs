mod rtmp_server;
mod storage;
mod rtmp_protocol;

use rtmp_server::RTMPServer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Example configuration
    let config = rtmp_server::ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 1935,
        mongo_uri: "mongodb://localhost:27017".to_string(),
    };

    // Initialize the RTMP server
    let mut rtmp_server = RTMPServer::new(config);
    rtmp_server.init_mongo().await;

    // Start the server
    rtmp_server.start().await
}
