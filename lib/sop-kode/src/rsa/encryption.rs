use num_bigint::BigUint;
use crate::calculate_chunk_size;

/// The PublicKey struct represents a public key in RSA encryption.
/// It contains two BigUint values, `n` and `e`.
#[derive(Clone, Debug)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

/// The PrivateKey struct represents a private key in RSA encryption.
/// It contains two BigUint values, `n` and `d`.
pub struct PrivateKey {
    n: BigUint,
    d: BigUint,
}

impl PublicKey {
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
    pub fn encrypt(&self, message: &BigUint, public_key: &PublicKey) -> BigUint {
        message.modpow(&public_key.e, &public_key.n)
    }
}

impl PrivateKey {
    /// Constructs a new PrivateKey with the given `n` and `d` values.
    ///
    /// # Arguments
    ///
    /// * `n` - The `n` value of the private key.
    /// * `d` - The `d` value of the private key.
    ///
    /// # Returns
    ///
    /// * `PrivateKey` - Returns a new PrivateKey.
    pub fn new(n: BigUint, d: BigUint) -> Self {
        Self { n, d }
    }

    /// The `decrypt` function is used to decrypt a message using a private key.
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - A BigUint value representing the message to be decrypted.
    ///
    /// # Returns
    ///
    /// * `BigUint` - Returns the decrypted message.
    pub fn decrypt(&self, ciphertext: &BigUint) -> BigUint {
        ciphertext.modpow(&self.d, &self.n)
    }

    /// The `get_chunk_size` function is used to calculate the chunk size for a message.
    ///
    /// # Returns
    ///
    /// * `usize` - Returns the chunk size.
    pub fn get_chunk_size(&self) -> usize {
        calculate_chunk_size(&self.n)
    }
}


