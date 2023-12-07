use num_bigint::BigUint;

pub struct Estimation {
    time: f64,
    unit: TimeUnit,
}

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Years,
    Millennia,
    Megaannum,
    Gigaannum,
}

/// The `base_n_to_base10` function converts a number from base `n` to base 10.
///
/// # Arguments
///
/// * `digits` - A vector of i64 integers representing the digits of the number in base `n`. The digits are in reverse order, with the least significant digit first. A digit of `-1` is interpreted as a zero digit.
/// * `base` - The base `n` of the number to be converted.
///
/// # Returns
///
/// * `i64` - The number converted to base 10.
///
/// # Example
///
/// ```
/// use sop_kode::rsa::*;
/// let digits = vec![1, 1, 2]; // Represents the number 211 in base 3
/// let base = 3;
/// let number = base_n_to_base10(&digits, base);
/// assert_eq!(number, 14); // 211 in base 3 is 22 in base 10
/// ```
pub fn base_n_to_base10(digits: &Vec<i64>, base: i64) -> i64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| {
        if digit == -1 {
            acc
        } else {
            acc + digit * base.pow(i as u32)
        }
    })
}

pub fn calculate_chunk_size(n: &BigUint) -> usize {
    // Get the size of n in bytes
    let n_size = n.bits() / 8;

    // Subtract a few bytes to leave room for padding
    //let padding = 11; // For PKCS#1 v1.5 padding
    let chunk_size = n_size;

    chunk_size as usize
}

/// The `chunk_message` function splits a string into chunks of bytes of a specified size.
///
/// # Arguments
///
/// * `s` - A string slice that represents the message to be chunked.
/// * `chunk_size` - The size of each chunk. The function will split the message into chunks of this size.
///
/// # Returns
///
/// * `Vec<Vec<u8>>` - A vector of byte vectors, where each inner vector represents a chunk of the original message.
///
/// # Example
///
/// ```
/// use sop_kode::rsa::*;
/// let message = "This is a test message.";
/// let chunk_size = 5;
/// let chunks = chunk_message(message, chunk_size);
/// ```
pub fn chunk_message(s: &str, chunk_size: usize) -> Vec<Vec<u8>> {
    let mut bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();

    // Calculate the amount of padding needed
    let padding = chunk_size - (bytes.len() % chunk_size);

    // Append all padding at once
    bytes.resize(bytes.len() + padding, 0);

    let chunks: Vec<Vec<u8>> = bytes
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    chunks
}


/// Estimates the time a brute force attack would take on a given encrypted message.
///
/// # Arguments
///
/// * `n` - The modulus of the rsa key used to encrypt the message.
///
/// # Returns
///
/// * `f64` - The estimated time in seconds.
pub fn estimate_brute_force_time(n: &BigUint) -> Estimation {
    let keys_per_second = 1_000_000f64; // Assume the attacker can try 1 million keys per second
    let key_space = n.bits() as f64; // Assume the key space is all integers less than n
    let time = 2f64.powf(key_space) / keys_per_second;

    if time < 60.0 {
        Estimation { time, unit: TimeUnit::Seconds }
    } else if time < 60.0 * 60.0 {
        Estimation { time: time / 60.0, unit: TimeUnit::Minutes }
    } else if time < 60.0 * 60.0 * 24.0 {
        Estimation { time: time / (60.0 * 60.0), unit: TimeUnit::Hours }
    } else if time < 60.0 * 60.0 * 24.0 * 365.25 {
        Estimation { time: time / (60.0 * 60.0 * 24.0), unit: TimeUnit::Days }
    } else {
        Estimation { time: time / (60.0 * 60.0 * 24.0 * 365.25), unit: TimeUnit::Years }
    }
}

pub fn format_duration(estimation: Estimation) -> String {
    match estimation.unit {
        TimeUnit::Seconds => format!("{:.2} seconds", estimation.time),
        TimeUnit::Minutes => format!("{:.2} minutes", estimation.time),
        TimeUnit::Hours => format!("{:.2} hours", estimation.time),
        TimeUnit::Days => format!("{:.2} days", estimation.time),
        TimeUnit::Years => format!("{:.2} years", estimation.time),
        TimeUnit::Millennia => format!("{:.2e} millennia", estimation.time),
        TimeUnit::Megaannum => format!("{:.2e} megaannum", estimation.time),
        TimeUnit::Gigaannum => format!("{:.2e} gigaannum", estimation.time),
    }
}
