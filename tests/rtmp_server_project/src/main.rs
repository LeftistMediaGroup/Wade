mod rtmp_server;
mod config;
mod streamer;
mod storage;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("rtmp://example.com/stream").await.unwrap();
    rtmp_server::process_stream(stream, None, None).await.unwrap();
}