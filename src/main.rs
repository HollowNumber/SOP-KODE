use std::collections::HashMap;
pub use sop_kode::*;

fn main() {
    let plaintext = "METTE";
    // danish alphabet as a vector
    let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                        "U", "V", "W", "X", "Y", "Z", "Æ", "Ø", "Å"];


    let encrypted = caesar_shift(plaintext, 3, alphabet);

    println!("Plaintext: {}", plaintext); // METTE
    println!("Encrypted: {}", encrypted); // PHWWH



    // RSA

    let message = "BARN";
    let k = 2; // Amount of vectors to be used
    let l = 3; // Amount of elements in each vector

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ";
    let alphabet_positions: HashMap<char, i64> = alphabet.chars().enumerate().map(|(i, c)| (c, i as i64)).collect();

    let chunked_message = chunk_message(message.chars().collect(), k, &alphabet_positions);

    println!("The chunked message is: {:?}", chunked_message);
    println!("The first term is: {}", base_n_to_base10(&chunked_message[0], 28));

    let pU = 17;
    let qU = 47;
    let nU = pU * qU;

    let eU = 7;
    let phi_nU = (pU - 1) * (qU - 1);
    println!("The value of phi_nA is: {}", phi_nU);

    // base of chunks[0]


    // Finding the modular inverse of eA modulo phi_nA
    let dU = mod_inverse(eU, phi_nU);

    println!("The modular inverse of {} modulo {} is: {}", eU, phi_nU, dU);
}


