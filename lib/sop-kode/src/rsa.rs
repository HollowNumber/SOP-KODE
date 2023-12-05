use std::collections::HashMap;
use std::ops::Shr;

use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, ToPrimitive, Zero};

fn shr_biguint_option(biguint: BigUint, option: Option<u64>) -> Result<BigUint, &'static str> {
    match option {
        Some(value) => Ok(biguint.shr(value)),
        None => Err("Cannot shift right by None"),
    }
}

/// Calculates the extended greatest common divisor (GCD) of two numbers.
///
/// The extended GCD is a triple (g, x, y), such that ax + by = g = gcd(a, b).
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
fn binary_extended_gcd(mut a: i64, mut b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let sign_a = if a < 0 { -1 } else { 1 };
    let sign_b = if b < 0 { -1 } else { 1 };

    a = a.abs();
    b = b.abs();

    let mut shift = 0;
    let (mut u, mut v) = (a, b);
    let (mut a1, mut b1, mut a2, mut b2) = (1, 0, 0, 1);

    while ((a | b) & 1) == 0 {
        a >>= 1;
        b >>= 1;
        shift += 1;
    }

    while u != 0 {
        while (u & 1) == 0 {
            u >>= 1;
            let (next_a1, next_b1) = if (a1 | b1) & 1 == 0 {
                (a1 >> 1, b1 >> 1)
            } else {
                ((a1 + b) >> 1, (b1 - a) >> 1)
            };
            a1 = next_a1;
            b1 = next_b1;
        }

        while (v & 1) == 0 {
            v >>= 1;
            let (next_a2, next_b2) = if (a2 | b2) & 1 == 0 {
                (a2 >> 1, b2 >> 1)
            } else {
                ((a2 + b) >> 1, (b2 - a) >> 1)
            };
            a2 = next_a2;
            b2 = next_b2;
        }

        if u >= v {
            u -= v;
            a1 -= a2;
            b1 -= b2;
        } else {
            v -= u;
            a2 -= a1;
            b2 -= b1;
        }
    }

    *x = a2 * sign_a;
    *y = b2 * sign_b;
    v << shift
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
fn miller_rabin(n: &BigUint, k: usize) -> bool {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Calculate n - 1
    let n_minus_one = n - BigUint::one();
    // Calculate d such that n - 1 = 2^r * d
    let d = match shr_biguint_option(n_minus_one.clone(), n_minus_one.trailing_zeros()) {
        Ok(value) => value,
        Err(_) => return false,
    };
    // Perform k trials
    'outer: for _ in 0..k {
        // Pick a random witness
        let a = rng.gen_biguint_range(&BigUint::from(2u64), &n_minus_one);
        // Calculate a^d mod n
        let mut x = a.modpow(&d, n);
        // If x is 1 or n - 1, continue to the next trial
        if x.is_one() || x == n_minus_one {
            continue;
        }
        // Repeat r times
        for _ in 0..n_minus_one.trailing_zeros().unwrap() {
            // Square x and reduce modulo n
            x = x.modpow(&BigUint::from(2u64), n);
            // If x is 1, return false (n is composite)
            if x.is_one() {
                return false;
            }
            // If x is n - 1, continue to the next trial
            if x == n_minus_one {
                continue 'outer;
            }
        }
        // If none of the conditions were met, return false (n is composite)
        return false;
    }
    // If no composite witness was found after k trials, return true (n is probably prime)
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
fn generate_prime(bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let n = rng.gen_biguint(bits as u64);
        if miller_rabin(&n, 5) {
            return n;
        }
    }
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
fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut x = 0;
    let mut y = 0;
    binary_extended_gcd(a, m, &mut x, &mut y);
    ((x % m) + m) % m
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
fn base_n_to_base10(digits: &Vec<i64>, base: i64) -> i64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| {
        if digit == -1 {
            acc
        } else {
            acc + digit * base.pow(i as u32)
        }
    })
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
fn chunk_message(
    chars: Vec<char>,
    chunk_size: usize,
    alphabet_positions: &HashMap<char, i64>,
) -> Vec<Vec<i64>> {
    let mut nums: Vec<i64> = chars.into_iter().map(|c| alphabet_positions[&c]).collect();

    // If the number of characters is not a multiple of chunk_size, append -1 to nums
    while nums.len() % chunk_size != 0 {
        nums.push(-1);
    }

    let chunks: Vec<Vec<i64>> = nums
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    chunks
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Instant;

    use super::*;

    #[test]
    fn compare_gcd_performance() {
        let mut x = 0;
        let mut y = 0;

        let start = Instant::now();
        for _ in 0..1_000_000 {
            binary_extended_gcd(3, 11, &mut x, &mut y);
        }
        let duration = start.elapsed();
        println!("Time elapsed in new gcd function is: {:?}", duration);

        // Assuming old_gcd is your previous implementation
        let start = Instant::now();
        for _ in 0..1_000_000 {
            extended_gcd(3, 11);
        }
        let duration = start.elapsed();
        println!("Time elapsed in old gcd function is: {:?}", duration);
    }

    #[test]
    fn binary_extended_gcd_returns_correct_values_for_positive_inputs() {
        let mut x = 0;
        let mut y = 0;
        assert_eq!(binary_extended_gcd(60, 24, &mut x, &mut y), 12);
    }

    #[test]
    fn binary_extended_gcd_returns_correct_values_for_negative_inputs() {
        let mut x = 0;
        let mut y = 0;
        assert_eq!(binary_extended_gcd(-60, 24, &mut x, &mut y), 12);
    }

    #[test]
    fn miller_rabin_returns_true_for_prime_number() {
        let prime = BigUint::from(13u64);
        assert!(miller_rabin(&prime, 5));
    }

    #[test]
    fn miller_rabin_returns_false_for_composite_number() {
        let composite = BigUint::from(15u64);
        assert!(!miller_rabin(&composite, 5));
    }

    #[test]
    fn generate_prime_returns_prime_number() {
        let prime = generate_prime(32);
        assert!(miller_rabin(&prime, 5));
    }

    #[test]
    fn extended_gcd_returns_correct_values() {
        assert_eq!(extended_gcd(48, 18), (6, -1, 3));
    }

    #[test]
    fn mod_inverse_returns_correct_value() {
        assert_eq!(mod_inverse(3, 11), 4);
    }

    #[test]
    fn mod_inverse_returns_correct_value_for_positive_inputs() {
        assert_eq!(mod_inverse(7, 26), 15);
    }

    #[test]
    fn mod_inverse_returns_correct_value_for_negative_inputs() {
        assert_eq!(mod_inverse(-7, 26), 11);
    }

    #[test]
    fn mod_inverse_returns_zero_for_non_coprime_inputs() {
        assert_eq!(mod_inverse(6, 26), 0);
    }

    #[test]
    fn mod_inverse_returns_one_for_coprime_inputs_equal_to_one() {
        assert_eq!(mod_inverse(1, 26), 1);
    }

    #[test]
    fn base_n_to_base10_returns_correct_value_for_base_2() {
        assert_eq!(base_n_to_base10(&vec![1, 0, 1], 2), 5);
    }

    #[test]
    fn base_n_to_base10_returns_correct_value_for_base_10() {
        assert_eq!(base_n_to_base10(&vec![1, 2, 3], 10), 123);
    }

    #[test]
    fn base_n_to_base10_returns_correct_value_for_base_16() {
        assert_eq!(base_n_to_base10(&vec![1, 2, 3], 16), 291);
    }

    #[test]
    fn base_n_to_base10_returns_correct_value_for_base_28() {
        assert_eq!(base_n_to_base10(&vec![1, 0], 28), 28);
    }

    #[test]
    fn chunk_message_returns_correct_chunks_for_even_number_of_characters() {
        let alphabet_positions: HashMap<char, i64> = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as i64))
            .collect();
        assert_eq!(
            chunk_message("HELLOO".chars().collect(), 3, &alphabet_positions),
            vec![vec![7, 4, 11], vec![11, 14, 14]]
        );
    }

    #[test]
    fn chunk_message_returns_correct_chunks_for_odd_number_of_characters() {
        let alphabet_positions: HashMap<char, i64> = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as i64))
            .collect();
        assert_eq!(
            chunk_message("HELLO".chars().collect(), 3, &alphabet_positions),
            vec![vec![7, 4, 11], vec![11, 14, -1]]
        );
    }

    #[test]
    fn calculate_totient_returns_correct_value() {
        let p = BigUint::from(13u64);
        let q = BigUint::from(17u64);
        assert_eq!(calculate_totient(&p, &q), BigUint::from(192u64));
    }

    #[test]
    fn encrypt_returns_correct_value() {
        let message = BigUint::from(7u64);
        let public_key = (BigUint::from(33u64), BigUint::from(3u64));
        assert_eq!(encrypt(message, public_key), BigUint::from(13u64));
    }

    #[test]
    fn decrypt_returns_correct_value() {
        let ciphertext = BigUint::from(13u64);
        let private_key = (BigUint::from(33u64), BigUint::from(7u64));
        assert_eq!(decrypt(ciphertext, private_key), BigUint::from(7u64));
    }
}
