#[derive(Debug)]
pub struct RTMPMessage {
    pub message_type: u8,
    pub message_length: u32,
    pub message_body: Vec<u8>,
}

impl RTMPMessage {
    pub fn from_bytes(data: &[u8]) -> Result<RTMPMessage, &'static str> {
        if data.len() < 5 {
            return Err("Invalid data length");
        }

        let message_type = data[0];
        let message_length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
        let message_body = data[5..].to_vec();

        Ok(RTMPMessage {
            message_type,
            message_length,
            message_body,
        })
    }
}

pub async fn process_rtmp_message(message: RTMPMessage) {
    // Example: Log the received message
    println!("{:?}", message);
    // You would add RTMP-specific processing here, such as streaming or command handling
}
