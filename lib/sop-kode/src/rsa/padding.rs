use rand::Rng;
use std::iter;
use num_bigint::BigUint;

/// The `pkcs1_pad` function is used to apply PKCS1 padding to a message.
///
/// # Arguments
///
/// * `message` - A BigUint value representing the message to be padded.
///
/// # Returns
///
/// * `Vec<u8>` - Returns the padded message.
pub fn pkcs1_pad(message: &BigUint) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut padded_message: Vec<u8> = iter::repeat_with(|| rng.gen()).take(8).collect();
    padded_message.extend_from_slice(&message.to_bytes_be());
    padded_message
}

/// The `pkcs1_unpad` function is used to remove PKCS1 padding from a message.
///
/// # Arguments
///
/// * `padded_message` - A BigUint value representing the padded message.
///
/// # Returns
///
/// * `BigUint` - Returns the unpadded message.
pub fn pkcs1_unpad(padded_message: &BigUint) -> BigUint {
    let unpadded_message = BigUint::from_bytes_be(&padded_message.to_bytes_be()[8..]);
    unpadded_message
}
