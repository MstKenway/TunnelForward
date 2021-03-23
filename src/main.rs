mod tunnel_core;

use rand::RngCore;
use rand::rngs::OsRng;
use tunnel_core::cipher;

fn main() {
    let message = "This is another test.";
    println!("{}", message);

    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 12] = [0; 12];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    // let mut rng = OsRng::new().ok().unwrap();
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);


    let mut encrypted_data = vec![0; message.len() + 16];
    let mut result = cipher::encrypt_aesgcm(message.as_bytes(), &mut encrypted_data, &key, &iv).ok().unwrap_or(0);
    println!("cipher text: {:?}", encrypted_data);

    // encrypted_data[0] = 0u8;

    let mut decrypted_data = vec![0; message.len()];
    result = cipher::decrypt_aesgcm(&encrypted_data[..], &mut decrypted_data, &key, &iv).ok().unwrap_or(0);
    let s = match String::from_utf8(decrypted_data) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("decrpyted text: {:}", s);

    // assert!(message.as_bytes() == &decrypted_data[..]);
}
