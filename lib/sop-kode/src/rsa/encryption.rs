use num_bigint::BigUint;
use super::keys::{PublicKey, PrivateKey};

/// The `encrypt` function is used to encrypt a message using a public key.
///
/// # Arguments
///
/// * `message` - A BigUint value representing the message to be encrypted.
/// * `public_key` - A PublicKey struct representing the public key to be used for encryption.
///
/// # Returns
///
/// * `BigUint` - Returns the encrypted message.
pub fn encrypt(message: &BigUint, public_key: &PublicKey) -> BigUint {
    message.modpow(&public_key.e, &public_key.n)
}

/// The `decrypt` function is used to decrypt a message using a private key.
///
/// # Arguments
///
/// * `ciphertext` - A BigUint value representing the message to be decrypted.
/// * `private_key` - A PrivateKey struct representing the private key to be used for decryption.
///
/// # Returns
///
/// * `BigUint` - Returns the decrypted message.
pub fn decrypt(ciphertext: &BigUint, private_key: &PrivateKey) -> BigUint {
    ciphertext.modpow(&private_key.d, &private_key.n)
}
