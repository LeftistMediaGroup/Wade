use aes::{ Aes128, BlockEncrypt, BlockDecrypt, NewBlockCipher };
use aes::cipher::{ generic_array::GenericArray, Block };
use hex;
use std::fmt;

const BLOCK_SIZE: usize = 16;

#[derive(Debug)]
pub struct EncryptionError;

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encryption error")
    }
}

fn pad(data: &str) -> Vec<u8> {
    let pad_size = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let mut padded = data.as_bytes().to_vec();
    padded.extend(vec![pad_size as u8; pad_size]);
    padded
}

fn unpad(data: &[u8]) -> Vec<u8> {
    let pad_size = data[data.len() - 1] as usize;
    data[..data.len() - pad_size].to_vec()
}

pub async fn encrypt(password: &str, data: &str) -> Result<String, EncryptionError> {
    let mut key = [0u8; 16];
    let password_bytes = password.as_bytes();
    let len = password_bytes.len().min(16);
    key[..len].copy_from_slice(&password_bytes[..len]);

    let cipher = Aes128::new(&GenericArray::from_slice(&key));

    let padded_data = pad(data);
    let mut encrypted = vec![0u8; padded_data.len()];

    for (i, chunk) in padded_data.chunks(BLOCK_SIZE).enumerate() {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&block);
    }

    Ok(hex::encode(encrypted))
}

pub async fn decrypt(password: &str, encrypted_hex: &str) -> Result<String, EncryptionError> {
    let mut key = [0u8; 16];
    let password_bytes = password.as_bytes();
    let len = password_bytes.len().min(16);
    key[..len].copy_from_slice(&password_bytes[..len]);

    let cipher = Aes128::new(&GenericArray::from_slice(&key));

    let encrypted_bytes = hex::decode(encrypted_hex).map_err(|_| EncryptionError)?;
    let mut decrypted = vec![0u8; encrypted_bytes.len()];

    for (i, chunk) in encrypted_bytes.chunks(BLOCK_SIZE).enumerate() {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE].copy_from_slice(&block);
    }

    let unpadded = unpad(&decrypted);
    String::from_utf8(unpadded).map_err(|_| EncryptionError)
}
