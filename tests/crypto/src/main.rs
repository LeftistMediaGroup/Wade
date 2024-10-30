use rsa::{ Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey };
use rsa::pkcs8::EncodePrivateKey;

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let priv_key2 = priv_key.to_pkcs8_pem();
}
