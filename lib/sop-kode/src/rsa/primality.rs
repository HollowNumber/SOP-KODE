use num_bigint::{BigUint, RandBigInt};
use num_traits::{FromPrimitive, One, Zero};

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
