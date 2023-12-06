use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt};
use num_traits::{FromPrimitive, One, Zero, Signed, ToPrimitive};
use rayon::prelude::*;
use std::collections::HashMap;

/// This function performs the binary extended Euclidean algorithm.
/// The binary extended Euclidean algorithm is an extension to the binary GCD algorithm,
/// and it is used to find the multiplicative inverse of a number modulo another number.
///
/// # Arguments
///
/// * `a` - A BigInt value representing the first number.
/// * `b` - A BigInt value representing the second number.
/// * `x` - A mutable reference to a BigInt that will hold the x value of the equation ax + by = gcd(a, b).
/// * `y` - A mutable reference to a BigInt that will hold the y value of the equation ax + by = gcd(a, b).
///
/// # Returns
///
/// * A BigInt representing the greatest common divisor (gcd) of `a` and `b`.
#[inline]
fn binary_extended_gcd(mut a: &BigInt, mut b: &BigInt, x: &mut BigInt, y: &mut BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();
    let sign_a = if a.sign() == Sign::Minus { BigInt::from(-1) } else { BigInt::one() };
    let sign_b = if b.sign() == Sign::Minus { BigInt::from(-1) } else { BigInt::one() };

    a = a.abs();
    b = b.abs();

    let mut shift = BigInt::zero();
    let (mut u, mut v) = (a.clone(), b.clone());
    let (mut a1, mut b1, mut a2, mut b2) = (BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one());

    while ((&a | &b) & BigInt::one()) == BigInt::zero() {
        a >>= 1;
        b >>= 1;
        shift += 1;
    }

    while u != BigInt::zero() {
        while (&u & BigInt::one()) == BigInt::zero() {
            u >>= 1;
            let (next_a1, next_b1) = if (&a1 | &b1) & BigInt::one() == BigInt::zero() {
                (a1.clone() >> 1, b1.clone() >> 1)
            } else {
                ((&a1 + &b) >> 1, (&b1 - &a) >> 1)
            };
            a1 = next_a1;
            b1 = next_b1;
        }

        while (&v & BigInt::one()) == BigInt::zero() {
            v >>= 1;
            let (next_a2, next_b2) = if (&a2 | &b2) & BigInt::one() == BigInt::zero() {
                (a2.clone() >> 1, b2.clone() >> 1)
            } else {
                ((&a2 + &b) >> 1, (&b2 - &a) >> 1)
            };
            a2 = next_a2;
            b2 = next_b2;
        }

        if u >= v {
            u -= &v;
            a1 -= a2.clone();
            b1 -= b2.clone();
        } else {
            v -= &u;
            a2 -= a1.clone();
            b2 -= b1.clone();
        }
    }

    *x = &a2 * &sign_a;
    *y = &b2 * &sign_b;
    v << shift.to_usize().unwrap()
}

/// The `miller_rabin` function is an implementation of the Miller-Rabin primality test.
/// The Miller-Rabin test is a probabilistic primality test: an algorithm which determines
/// whether a given number is likely to be prime, similar to the Fermat primality test
/// and the Solovay–Strassen primality test.
///
/// # Arguments
///
/// * `n` - A BigUint value representing the number to be tested for primality.
/// * `k` - The number of rounds of testing to perform. The higher the value of `k`,
///          the more accurate the test is. A common value for `k` is 5.
///
/// # Returns
///
/// * `bool` - Returns `true` if `n` is likely to be prime, and `false` otherwise.
#[inline]
pub fn miller_rabin(n: &BigUint, k: usize) -> bool {
    let small_primes = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    for &prime in small_primes {
        if n == &BigUint::from(prime as usize) {
            return true;
        } else if n % prime as usize == BigUint::zero() {
            return false;
        }
    }

    let n_minus_one = n - BigUint::one();
    let d = n_minus_one.clone() >> n_minus_one.trailing_zeros().unwrap() as usize;

    'outer: for i in 0..k {
        let a = BigUint::from_u32(small_primes[i % small_primes.len()] as u32).unwrap();
        let mut x = a.modpow(&d, n);
        if x.is_one() || x == n_minus_one {
            continue;
        }
        for _ in 0..n_minus_one.trailing_zeros().unwrap() {
            x = x.modpow(&BigUint::from(2u64), n);
            if x.is_one() {
                return false;
            }
            if x == n_minus_one {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// The `generate_prime` function is used to generate a prime number of a specified bit size.
///
/// # Arguments
///
/// * `bits` - The number of bits in the prime number to be generated. The function will generate a prime number that is approximately this size.
///
/// # Returns
///
/// * `BigUint` - Returns a prime number of approximately `bits` bits.
///
/// # Example
///
/// ```rs
/// use sop_kode::rsa::*;
/// let prime = generate_prime(512);
/// assert!(miller_rabin(&prime, 5));
/// ```
#[inline]
pub fn generate_prime(bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_biguint(bits as u64);
    let zero = BigUint::zero();
    // Ensure n is odd
    if &n % 2usize == zero {
        n += 1usize;
    }

    while !miller_rabin(&n, 5) {
        // Increment by 2 to ensure n stays odd
        n += 2usize;
    }

    n
}

/// The `extended_gcd` function is an implementation of the Extended Euclidean Algorithm.
/// The Extended Euclidean Algorithm is an extension to the Euclidean Algorithm, which is used to find
/// the greatest common divisor (gcd) of two numbers. In addition to finding the gcd, the Extended Euclidean Algorithm
/// also finds coefficients that satisfy Bézout's identity, which states that the gcd of two numbers can be expressed
/// as a linear combination of those two numbers.
///
/// This function is deprecated and `binary_extended_gcd` should be used instead.
///
/// # Arguments
///
/// * `a` - An i64 integer representing the first number.
/// * `b` - An i64 integer representing the second number.
///
/// # Returns
///
/// * A tuple of three i64 integers:
///   * The first integer is the greatest common divisor (gcd) of `a` and `b`.
///   * The second and third integers are the coefficients that satisfy Bézout's identity.
///
/// # Deprecated
///
/// This function is deprecated and `binary_extended_gcd` should be used instead.
#[deprecated(note = "Use `binary_extended_gcd` instead.")]
#[inline]
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x, y) = extended_gcd(b % a, a);
    (g, y - (b / a) * x, x)
}

/// The `mod_inverse` function calculates the modular multiplicative inverse of a number.
/// The modular multiplicative inverse of `a` modulo `m` is an integer `x` such that
/// the product `ax` is congruent to `1` modulo `m`. If the modular multiplicative inverse
/// of `a` modulo `m` exists, the function returns it. Otherwise, it returns 0.
///
/// The function uses the Extended Euclidean Algorithm to find the inverse.
///
/// # Arguments
///
/// * `a` - A BigInt value for which to find the modular multiplicative inverse.
/// * `m` - The modulus.
///
/// # Returns
///
/// * `BigInt` - The modular multiplicative inverse of `a` modulo `m`, or 0 if it does not exist.
pub fn mod_inverse(a: BigInt, m: BigInt) -> BigInt {
    let mut x = BigInt::zero();
    let mut y = BigInt::zero();
    let gcd = binary_extended_gcd(&a, &m, &mut x, &mut y);
    if gcd != BigInt::one() {
        // a and m are not coprime, return early
        return BigInt::zero();
    }
    // Perform modulo operation only once
    let result = (x % &m) + &m;
    if result >= m {
        result - &m
    } else {
        result
    }
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

/// The `encrypt_message` function encrypts a message using RSA encryption.
///
/// # Arguments
///
/// * `message` - A string slice that represents the message to be encrypted.
/// * `public_key` - A reference to a tuple representing the RSA public key. The first element of the tuple is the modulus `n`, and the second element is the public exponent `e`.
///
/// # Returns
///
/// * `Vec<BigUint>` - A vector of `BigUint`s, where each `BigUint` is the encrypted form of a chunk of the original message.
///
/// # Example
///
/// ```
/// use sop_kode::rsa::*;
/// use num_bigint::BigUint;
///
/// let message = "This is a test message.";
/// let public_key = generate_keys(1024).unwrap().0;
/// let encrypted_message = encrypt_message(message, &public_key);
/// ```
pub fn encrypt_message(message: &str, public_key: &(BigUint, BigUint)) -> Vec<BigUint> {
    let (ref n, _) = public_key;

    // Calculate the chunk size
    let chunk_size = calculate_chunk_size(&n);

    // Convert the string to chunks of bytes
    let chunks = chunk_message(message, chunk_size);

    // Convert each chunk of bytes to a BigUint and encrypt it
    let encrypted_chunks: Vec<BigUint> = chunks.into_iter()
        .map(|chunk| {
            let chunk_biguint = BigUint::from_bytes_be(&chunk);
            encrypt(chunk_biguint, public_key)
        })
        .collect();

    encrypted_chunks
}

/// The `decrypt_message` function decrypts a message using RSA decryption.
///
/// # Arguments
///
/// * `encrypted_message` - A vector of `BigUint`s, where each `BigUint` is the encrypted form of a chunk of the original message.
/// * `private_key` - A tuple representing the RSA private key. The first element of the tuple is the modulus `n`, and the second element is the private exponent `d`.
///
/// # Returns
///
/// * `String` - The decrypted message.
///
/// # Example
///
/// ```
/// use sop_kode::rsa::*;
/// use num_bigint::BigUint;
/// let encrypted_message = vec![BigUint::from(1234567890u64)];
/// let private_key = (BigUint::from(33u64), BigUint::from(7u64));
/// let message = decrypt_message(encrypted_message, private_key);
// /// ```
pub fn decrypt_message(encrypted_message: Vec<BigUint>, private_key: (BigUint, BigUint)) -> String {
    // Decrypt each chunk separately
    let decrypted_chunks: Vec<Vec<u8>> = encrypted_message.into_iter()
        .map(|chunk| {
            let decrypted_chunk = decrypt(chunk, &private_key);
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


/// The `calculate_totient` function calculates the Euler's totient (or phi) function for the product of two prime numbers `p` and `q`.
/// Euler's totient function counts the positive integers up to a given integer `n` that are relatively prime to `n`.
///
/// # Arguments
///
/// * `p` - A reference to a BigUint that represents the first prime number.
/// * `q` - A reference to a BigUint that represents the second prime number.
///
/// # Returns
///
/// * `BigUint` - The value of the Euler's totient function for `p * q`.
fn calculate_totient(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}

/// The `encrypt` function encrypts a message using RSA encryption.
///
/// # Arguments
///
/// * `message` - A BigUint that represents the message to be encrypted.
/// * `public_key` - A reference to a tuple representing the RSA public key. The first element of the tuple is the modulus `n`, and the second element is the public exponent `e`.
///
/// # Returns
///
/// * `BigUint` - The encrypted message.
pub fn encrypt(message: BigUint, public_key: &(BigUint, BigUint)) -> BigUint {
    let (n, e) = public_key;
    message.modpow(&e, &n)
}

/// The `decrypt` function decrypts a message using RSA decryption.
///
/// # Arguments
///
/// * `ciphertext` - A BigUint that represents the encrypted message to be decrypted.
/// * `private_key` - A tuple representing the RSA private key. The first element of the tuple is the modulus `n`, and the second element is the private exponent `d`.
///
/// # Returns
///
/// * `BigUint` - The decrypted message.
pub fn decrypt(ciphertext: BigUint, private_key: &(BigUint, BigUint)) -> BigUint {
    let (n, d) = private_key;
    ciphertext.modpow(&d, &n)
}

/// The `generate_keys` function generates a pair of RSA keys (public and private).
///
/// # Arguments
///
/// * `bits` - The number of bits in the RSA modulus `n`. The function will generate keys for which `n` is approximately this size.
///
/// # Returns
///
/// * `Option<((BigUint, BigUint), (BigUint, BigUint))>` - Returns a pair of RSA keys (public and private), where each key is a pair of `BigUint`s. The first element of each key is the modulus `n`, and the second element is the exponent (`e` for the public key, `d` for the private key). If the generated primes `p` and `q` are equal, the function returns `None` to indicate that key generation failed.
///
/// # Example
///
/// ```
/// use sop_kode::rsa::*;
/// let keys = generate_keys(1024);
/// match keys {
///     Some(((n, e), (n_prime, d))) => {
///         // Use the keys...
///     },
///     None => {
///         // Key generation failed...
///     },
/// }
/// ```
pub fn generate_keys(bits: usize) -> Option<((BigUint, BigUint), (BigUint, BigUint))> {
    let (p, q) = rayon::join(|| generate_prime(bits / 2), || generate_prime(bits / 2));

    if p == q {
        return None;
    }

    let n = p.clone() * q.clone();
    let phi = calculate_totient(&p, &q);

    let e = BigUint::from(65537u64); // Commonly used public exponent

    let d = mod_inverse(e.clone().to_bigint().unwrap(), phi.to_bigint().unwrap());

    Some(((n.clone(), e), (n, d.to_biguint().unwrap())))
}
