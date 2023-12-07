use std::collections::HashMap;

pub use sop_kode::*;

fn main() {
    let plaintext = "Hey bro, this is a test message. I hope you like it!";
    let rsa = RSA::new(48).unwrap();
    let encrypted = rsa.encrypt(plaintext);

    println!("Encrypted: {:?}", encrypted);

    let time = estimate_brute_force_time(&rsa.public_key.n);
    println!("Estimated time to brute force: {}", format_duration(time));

    let decrypted = rsa.decrypt_message(encrypted);
    println!("Decrypted: {}", decrypted);
}
