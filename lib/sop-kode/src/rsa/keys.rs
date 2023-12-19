use super::generate_prime;
use super::calculate_totient;
use super::mod_inverse;
use super::encryption::{PrivateKey, PublicKey};

use num_bigint::{BigUint, ToBigInt};
use super::chunk_message;

/// The RSA struct represents an RSA encryption/decryption system.
pub struct RSA {
    pub public_key: PublicKey,
    private_key: PrivateKey,
}

impl RSA {
    /// Constructs a new RSA system with the given number of bits.
    ///
    /// # Arguments
    ///
    /// * `bits` - The number of bits for the RSA system.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - Returns an RSA system if successful, or None if the generated primes are equal.
    pub fn new(bits: usize) -> Option<Self> {
        let (p, q) = rayon::join(|| generate_prime(bits / 2), || generate_prime(bits / 2));
        

        if p == q {
            return None;
        }


        let n = p.clone() * q.clone();
        let phi = calculate_totient(&p, &q);

        let e = BigUint::from(65537u64); // Commonly used public exponent

        let d = mod_inverse(e.clone().to_bigint().unwrap(), phi.to_bigint().unwrap());

        Some(Self {
            public_key: PublicKey { n: n.clone(), e },
            private_key: PrivateKey::new(n, d.to_biguint().unwrap()),
        })
    }

    /// Encrypts a message using the given public key.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to encrypt.
    /// * `public_key` - The public key to use for encryption.
    ///
    /// # Returns
    ///
    /// * `Vec<BigUint>` - Returns the encrypted message as a vector of BigUint.
    pub fn encrypt_message(&self, message: &str, public_key: PublicKey) -> Vec<BigUint> {

        // Calculate the chunk size
        let chunk_size = &self.private_key.get_chunk_size();

        // Convert the string to chunks of bytes
        let chunks = chunk_message(message, *chunk_size);

        // Convert each chunk of bytes to a BigUint and encrypt it
        let encrypted_chunks: Vec<BigUint> = chunks.into_iter()
            .map(|chunk| {
                let chunk_biguint = BigUint::from_bytes_be(&chunk);
                public_key.encrypt(&chunk_biguint, &public_key)
            })
            .collect();

        encrypted_chunks
    }

    /// Decrypts an encrypted message.
    ///
    /// # Arguments
    ///
    /// * `encrypted_message` - The encrypted message to decrypt.
    ///
    /// # Returns
    ///
    /// * `String` - Returns the decrypted message as a string.
    pub fn decrypt_message(&self, encrypted_message: Vec<BigUint>) -> String {
        // Decrypt each chunk separately
        let decrypted_chunks: Vec<Vec<u8>> = encrypted_message.into_iter()
            .map(|chunk| {
                let decrypted_chunk = self.private_key.decrypt(&chunk);
                decrypted_chunk.to_bytes_be()
            })
            .collect();

        // Concatenate the decrypted chunks together to recover the original message
        let decrypted_message: Vec<u8> = decrypted_chunks.into_iter().flatten().collect();
        let decrypted_message = String::from_utf8(decrypted_message).unwrap();

        // Remove any trailing null characters from the decrypted message
        let decrypted_message = decrypted_message.trim_end_matches('\0');

        decrypted_message.to_string()
    }
}
