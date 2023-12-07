use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Zero, Signed, ToPrimitive};



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
pub fn binary_extended_gcd(mut a: &BigInt, mut b: &BigInt, x: &mut BigInt, y: &mut BigInt) -> BigInt {
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
pub fn calculate_totient(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
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
