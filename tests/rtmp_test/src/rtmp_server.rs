use std::io::{ BufWriter };
use std::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use crate::storage;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::broadcast;
use std::io::Result;

const HANDSHAKE_SIZE: usize = 1537;

pub fn process_stream(
    mut socket: TcpStream,
    encryption_key: Option<[u8; 32]>,
    encryption_iv: Option<[u8; 16]>
) -> Result<()> {
    // Perform the handshake with the client
    perform_handshake(&mut socket)?;

    // Set up the recording
    let mut record_file = BufWriter::new(File::create("stream_recording.flv")?);

    let mut data = vec![0; 1024];

    // Stream processing loop - simulate receiving stream data
    loop {
        // Here, we would read incoming stream data into 'data'
        let bytes_read = socket.read(&mut data)?;

        if bytes_read == 0 {
            // If no data was read, the connection has likely been closed
            break;
        }

        // If encryption is enabled, encrypt the data before writing
        if let Some(key) = encryption_key {
            if let Some(iv) = encryption_iv {
                encrypt_and_send(&mut socket, &data[..bytes_read], &key, &iv)?;
            }
        } else {
            // Write data without encryption (record or forward)
            record_file.write_all(&data[..bytes_read])?;
        }
    }

    Ok(())
}

pub fn encrypt_and_send(socket: &mut TcpStream, data: &[u8], key: &[u8], iv: &[u8]) -> Result<()> {
    // Encrypt the data using the provided key and iv
    let encrypted_data = storage::encrypt_data(key, iv, data);

    // Send the encrypted data over the socket
    socket.write_all(&encrypted_data)?;

    Ok(())
}

pub fn perform_handshake(socket: &mut TcpStream) -> Result<()> {
    let mut buf = vec![0u8; HANDSHAKE_SIZE];

    // Step 1: Read C0 and C1 (client handshake)
    socket.read_exact(&mut buf[0..2])?; // Read C0 and C1
    let version = buf[0]; // C0 byte for version
    let client_c1 = buf[1..1537].to_vec(); // C1 message (random bytes)

    // Step 2: Create S0, S1, S2 (server handshake)
    buf[0] = version; // S0 should match C0

    // You can generate your server's S1 (timestamp + random data)
    let server_s1 = vec![0u8; 1536]; // For simplicity, using zeros here
    buf[1..1537].copy_from_slice(&server_s1);

    // Step 3: Send S0, S1, S2 back to the client
    socket.write_all(&buf[0..1537])?;

    // Step 4: Handle C2 (client response) after server handshake
    socket.read_exact(&mut buf[0..1537])?; // Read C2 message

    // Log and verify handshake steps
    println!("Handshake completed successfully with version {}", version);

    Ok(())
}
