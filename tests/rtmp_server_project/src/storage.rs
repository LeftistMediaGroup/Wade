use aes::{ Aes256, cipher::{ BlockEncryptMut, BlockDecryptMut } };
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aes::Aes256Gcm;
use aes_gcm::cipher::encryptor::Encryptor;
use aes_gcm::cipher::decryptor::Decryptor;
use aes_gcm::aead::{ generic_array::typenum::U32, Aes256GcmEncrypt, Aes256Gcm };
use rand::Rng;

pub fn encrypt_data(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(iv);
    let cipher = Aes256Gcm::new(key, iv);

    let mut encrypted_data = data.to_vec();
    cipher.encrypt(&[], &mut encrypted_data).unwrap();
    encrypted_data
}

pub fn decrypt_data(key: &[u8], iv: &[u8], encrypted_data: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(iv);
    let cipher = Aes256Gcm::new(key, iv);

    let mut decrypted_data = encrypted_data.to_vec();
    cipher.decrypt(&[], &mut decrypted_data).unwrap();
    decrypted_data
}
