use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt};
use num_traits::{FromPrimitive, One, Zero, Signed, ToPrimitive};
use rayon::prelude::*;
use std::collections::HashMap;

/// Calculates the extended greatest common divisor (GCD) of two numbers.
///
/// The extended GCD is a triple (g, x, y), such that ax + by = g = gcd(a, b).
///
/// This function has a time complexity of `O(log(min(a, b)))`.
///
/// # Arguments
///
/// * `a` - An integer input.
/// * `b` - Another integer input.
/// * `x` - A mutable reference to an integer.
/// * `y` - A mutable reference to an integer.
///
/// # Returns
///
/// * A tuple (g, x, y) where g is the GCD of a and b, and x and y satisfy the equation ax + by = g.
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

/// Performs the Miller-Rabin primality test.
///
/// The Miller-Rabin test is a probabilistic test to check if a number is a prime number.
/// It's based on the claim that if `n` is a prime number and `n-1 = 2^r * d` for an odd integer `d` and non-negative integer `r`,
/// then for any integer `a` such that `2 ≤ a ≤ n - 2`, one of the following is true:
/// 1. `a^d ≡ 1 (mod n)`
/// 2. `a^(2^i * d) ≡ -1 (mod n)` for some `i` where `0 ≤ i < r`
///
/// # Arguments
///
/// * `n` - The number to be tested for primality.
/// * `k` - The number of random witnesses to test.
///
/// # Returns
///
/// * `true` if `n` is probably prime, `false` if `n` is composite.
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

/// Generates a prime number of a given bit size.
///
/// This function generates a random bit integer and increments it by 2 until a prime number is found.
/// It uses the `miller_rabin` function for primality testing.
///
/// # Arguments
///
/// * `bits` - The number of bits in the prime number to be generated.
///
/// # Returns
///
/// * A prime number of the specified bit size.
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

/// Calculates the extended greatest common divisor (GCD) of two numbers.
///
/// The extended GCD is a triple (g, x, y), such that ax + by = g = gcd(a, b).
///
/// # Arguments
///
/// * `a` - An integer input.
/// * `b` - Another integer input.
///
/// # Returns
///
/// * A tuple (g, x, y) where g is the GCD of a and b, and x and y satisfy the equation ax + by = g.
///
/// # Deprecated
///
/// This function is deprecated and may be removed in a future version.
/// Use `binary_extended_gcd` instead.
#[deprecated(note = "Use `binary_extended_gcd` instead.")]
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x, y) = extended_gcd(b % a, a);
    (g, y - (b / a) * x, x)
}

/// Calculates the modular multiplicative inverse of two numbers.
///
/// The modular multiplicative inverse of a modulo m is an integer x such that the product ax is congruent to 1 modulo m.
///
/// # Arguments
///
/// * `a` - An integer input.
/// * `m` - The modulus.
///
/// # Returns
///
/// * The modular multiplicative inverse of a modulo m.
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

/// Converts a number from base n to base 10.
///
/// # Arguments
///
/// * `digits` - A vector of digits in base n.
/// * `base` - The base of the input number.
///
/// # Returns
///
/// * The input number converted to base 10.
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

    /// Chunks a message into groups of a certain size.
///
/// # Arguments
///
/// * `chars` - A vector of characters to be chunked.
/// * `chunk_size` - The size of each chunk.
/// * `alphabet_positions` - A mapping from characters to their positions in the alphabet.
///
/// # Returns
///
/// * A vector of chunks, where each chunk is a vector of integers representing the positions of the characters in the alphabet.
pub fn chunk_message(s: &str, chunk_size: usize) -> Vec<Vec<u8>> {
    let mut bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();

    // If the number of characters is not a multiple of chunk_size, append 0 to bytes
    while bytes.len() % chunk_size != 0 {
        bytes.push(0);
    }

    let chunks: Vec<Vec<u8>> = bytes
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    chunks
}


pub fn encrypt_message(message: &str, public_key: (BigUint, BigUint)) -> Vec<BigUint> {
    let (ref n, _) = public_key;

    // Calculate the chunk size
    let chunk_size = calculate_chunk_size(&n);

    // Convert the string to chunks of bytes
    let chunks = chunk_message(message, chunk_size);

    // Convert each chunk of bytes to a BigUint and encrypt it
    let encrypted_chunks: Vec<BigUint> = chunks.into_iter()
        .map(|chunk| {
            let chunk_biguint = BigUint::from_bytes_be(&chunk);
            encrypt(chunk_biguint, public_key.clone())
        })
        .collect();

    encrypted_chunks
}

pub fn decrypt_message(encrypted_message: Vec<BigUint>, private_key: (BigUint, BigUint)) -> String {
    // Decrypt each chunk separately
    let decrypted_chunks: Vec<Vec<u8>> = encrypted_message.into_iter()
        .map(|chunk| {
            let decrypted_chunk = decrypt(chunk, private_key.clone());
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

/// Calculates the totient function for two numbers.
///
/// The totient function, also known as Euler's totient function, counts the positive integers up to a given integer n that are relatively prime to n.
///
/// # Arguments
///
/// * `p` - A prime number.
/// * `q` - Another prime number.
///
/// # Returns
///
/// * The totient function of the product of `p` and `q`.
fn calculate_totient(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}

/// Encrypts a message using a public key.
///
/// The encryption is done by raising the message to the power of `e` modulo `n`.
///
/// # Arguments
///
/// * `message` - The message to be encrypted, represented as a `BigUint`.
/// * `public_key` - A tuple `(n, e)`, where `n` is the modulus and `e` is the exponent.
///
/// # Returns
///
/// * The encrypted message, represented as a `BigUint`.
pub fn encrypt(message: BigUint, public_key: (BigUint, BigUint)) -> BigUint {
    let (n, e) = public_key;
    message.modpow(&e, &n)
}

/// Decrypts a ciphertext using a private key.
///
/// The decryption is done by raising the ciphertext to the power of `d` modulo `n`.
///
/// # Arguments
///
/// * `ciphertext` - The ciphertext to be decrypted, represented as a `BigUint`.
/// * `private_key` - A tuple `(n, d)`, where `n` is the modulus and `d` is the exponent.
///
/// # Returns
///
/// * The decrypted message, represented as a `BigUint`.
pub fn decrypt(ciphertext: BigUint, private_key: (BigUint, BigUint)) -> BigUint {
    let (n, d) = private_key;
    ciphertext.modpow(&d, &n)
}


pub fn generate_keys(bits: usize) -> Option<((BigUint, BigUint), (BigUint, BigUint))> {
    let p = generate_prime(bits / 2);
    let q = generate_prime(bits / 2);
    if p == q {
        return None;
    }

    let n = &p * &q;
    let phi = calculate_totient(&p, &q);

    let e = BigUint::from(65537u64); // Commonly used public exponent

    let d = mod_inverse(e.clone().to_bigint().unwrap(), phi.to_bigint().unwrap());

    Some(((n.clone(), e), (n, d.to_biguint().unwrap())))
}
