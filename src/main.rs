use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::io::Write;

fn create_files(start: usize, end: usize, step: usize) -> std::io::Result<()> {
    for i in (start..=end).step_by(step) {
        let mut file = File::create(format!("lib/sop-kode/benches/text files/file_{}.txt", i))?;
        let data: String = std::iter::repeat('a').take(i).collect();
        file.write_all(data.as_bytes())?;
    }
    Ok(())
}



use sop_kode::*;

fn main() {
    let user: RSA = RSA::new(2048).expect("Failed to create RSA");
    let mut user2: RSA = RSA::new(2048).expect("Failed to create RSA");

    let message = "This is a test message.";




    let encrypted_message = user.encrypt_message(message, user2.public_key.clone());
    let decrypted_message = user2.decrypt_message(encrypted_message.clone());
    println!("Encrypted message: {:?}", &encrypted_message);
    println!("Decrypted message: {}", &decrypted_message);

}
