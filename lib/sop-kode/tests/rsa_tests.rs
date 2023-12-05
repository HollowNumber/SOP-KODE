use num_bigint::BigUint;

use sop_kode::rsa::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miller_rabin_returns_true_for_prime_number() {
        let prime = BigUint::from(13u64);
        assert!(miller_rabin(&prime, 100));
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
        assert_eq!(mod_inverse(1, 29), 1);
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
    fn decrypt_returns_correct_value() {
        let ciphertext = BigUint::from(13u64);
        let private_key = (BigUint::from(33u64), BigUint::from(7u64));
        assert_eq!(decrypt(ciphertext, private_key), BigUint::from(7u64));
    }
}
