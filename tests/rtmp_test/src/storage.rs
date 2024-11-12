use openssl::symm::{ Cipher, encrypt, decrypt };
use openssl::rand::rand_bytes;

pub fn encrypt_data(data: &[u8]) -> Vec<u8> {
    let key = b"0123456789abcdef"; // 16-byte key (AES-128)
    let iv = b"0123456789abcdef"; // 16-byte IV

    encrypt(Cipher::aes_128_cbc(), key, Some(iv), data).unwrap()
}

pub fn decrypt_data(data: &[u8]) -> Vec<u8> {
    let key = b"0123456789abcdef"; // 16-byte key (AES-128)
    let iv = b"0123456789abcdef"; // 16-byte IV

    decrypt(Cipher::aes_128_cbc(), key, Some(iv), data).unwrap()
}
