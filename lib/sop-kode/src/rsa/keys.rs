use num_bigint::{BigUint, ToBigInt};
use super::generate_prime;
use super::calculate_totient;
use super::mod_inverse;
use rayon::prelude::*;

pub struct PublicKey {
    pub n: BigUint,
    pub(crate) e: BigUint,
}

pub struct PrivateKey {
    pub(crate) n: BigUint,
    pub(crate) d: BigUint,
}

pub struct RSA {
    pub public_key: PublicKey,
    private_key: PrivateKey,
}

impl RSA {
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
            private_key: PrivateKey { n, d: d.to_biguint().unwrap() },
        })
    }

    pub fn encrypt(&self, message: BigUint) -> BigUint {
        let PublicKey { n, e } = &self.public_key;
        message.modpow(e, n)
    }

    pub fn decrypt(&self, ciphertext: BigUint) -> BigUint {
        let PrivateKey { n, d } = &self.private_key;
        ciphertext.modpow(d, n)
    }
}
