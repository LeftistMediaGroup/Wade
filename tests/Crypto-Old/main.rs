use encryptfile as ef;

fn main() {
    // Encrypt
    let mut in_file = std::env::var("HOME").unwrap();
    in_file.push_str("/test.webm");
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
        .output_stream(ef::OutputStream::File("/tmp/encrypted_test.ef".to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text("iloveyou".to_owned(), ef::scrypt_defaults()))
        .encrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));

    // Decrypt
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File("/tmp/encrypted_test.ef".to_owned()))
        .output_stream(ef::OutputStream::File("/tmp/decrypted_test.webm".to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(
            ef::PasswordType::Text("iloveyou".to_owned(), ef::PasswordKeyGenMethod::ReadFromFile)
        )
        .decrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
}
