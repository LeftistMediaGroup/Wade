use crate::storage::Storage;
use mongodb::{ Client, options::ClientOptions, Collection };
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use std::fs::{ self, File };
use std::io::{ BufWriter, Result };
use crate::rtmp_server::perform_handshake;
pub struct Stream {
    pub name: String,
    subscribers: Vec<broadcast::Sender<Vec<u8>>>,
    record_file: Option<BufWriter<File>>,
}

impl Stream {
    pub async fn handle_client(
        mut socket: TcpStream,
        streams: Arc<Mutex<HashMap<String, Stream>>>,
        mongo_client: Client,
        encryption_key: Option<[u8; 32]>
    ) -> Result<()> {
        // Perform the handshake first
        perform_handshake(&mut socket).await?;

        // Start the stream processing
        let stream_name = "example_stream";
        let custom_path = format!("./streams/{}", stream_name);

        // Handle the push
        Self::handle_push(
            stream_name.to_string(),
            streams,
            custom_path,
            mongo_client,
            encryption_key
        ).await
    }

    pub async fn handle_push(
        stream_name: String,
        streams: Arc<Mutex<HashMap<String, Stream>>>,
        custom_path: String,
        mongo_client: Client,
        encryption_key: Option<[u8; 32]>
    ) -> Result<()> {
        let (tx, _) = broadcast::channel(16);

        fs::create_dir_all(&custom_path).expect("Failed to create recording directory");
        let file_path = format!("{}/{}.flv", custom_path, stream_name);
        let file = File::create(&file_path)?;
        let mut record_file = BufWriter::new(file);

        let stream = Stream {
            name: stream_name.clone(),
            subscribers: vec![tx.clone()],
            record_file: Some(record_file),
        };

        streams.lock().unwrap().insert(stream_name.clone(), stream);
        println!("Started publishing stream: {}", stream_name);

        let mut data = vec![0; 1024];
        loop {
            match tx.send(data.clone()) {
                Ok(_) => {
                    Storage::encrypt_and_store(
                        &mongo_client,
                        &data,
                        &stream_name,
                        encryption_key.clone()
                    ).await;
                }
                Err(_) => {
                    println!("Client disconnected from stream: {}", stream_name);
                    break;
                }
            }
        }

        Ok(())
    }
}
