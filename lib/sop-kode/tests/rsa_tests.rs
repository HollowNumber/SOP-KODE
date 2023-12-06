use num_bigint::{BigInt, BigUint};

use sop_kode::rsa::*;

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;
    use num_traits::{FromPrimitive, One, Zero};

    use super::*;

    mod miller_rabin_tests {
        use super::*;

        #[test]
        fn returns_true_for_prime_number() {
            let prime = BigUint::from_i32(13).unwrap();
            assert!(miller_rabin(&prime, 100));
        }

        #[test]
        fn returns_false_for_composite_number() {
            let composite = BigUint::from_i32(15).unwrap();
            assert!(!miller_rabin(&composite, 5));
        }
    }

    #[test]
    fn mod_inverse_returns_correct_value_for_positive_inputs() {
        assert_eq!(mod_inverse(BigInt::from(7), BigInt::from(26)), BigInt::from(15));
    }

    #[test]
    fn mod_inverse_returns_correct_value_for_negative_inputs() {
        assert_eq!(mod_inverse(BigInt::from(-7), BigInt::from(26)), BigInt::from(11));
    }

    #[test]
    fn mod_inverse_returns_zero_for_non_coprime_inputs() {
        assert_eq!(mod_inverse(BigInt::from(6), BigInt::from(26)), BigInt::zero());
    }

    #[test]
    fn mod_inverse_returns_one_for_coprime_inputs_equal_to_one() {
        assert_eq!(mod_inverse(BigInt::from(1), BigInt::from(29)), BigInt::one());
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

    mod chunk_message_tests {
        use super::*;

        #[test]
        fn returns_correct_chunks() {
            let message = "Hello, world!";
            let chunk_size = 5;
            let expected_chunks: Vec<Vec<u8>> = vec![
                vec![72, 101, 108, 108, 111],
                vec![44, 32, 119, 111, 114],
                vec![108, 100, 33, 0, 0],
            ];

            assert_eq!(chunk_message(message, chunk_size), expected_chunks);
        }

        #[test]
        fn returns_correct_chunks_for_chunk_size_2() {
            let message = "Hello, world!";
            let chunk_size = 2;
            let expected_chunks: Vec<Vec<u8>> = vec![
                vec![72, 101],
                vec![108, 108],
                vec![111, 44],
                vec![32, 119],
                vec![111, 114],
                vec![108, 100],
                vec![33, 0],
            ];

            assert_eq!(chunk_message(message, chunk_size), expected_chunks);
        }
    }

    mod message_encryption_tests {
        use super::*;

        #[test]
        fn encrypt_and_decrypt_short_message() {
            let (public_key, private_key) = generate_keys(1024).unwrap();
            let message = "Hello";

            let encrypted_message = encrypt_message(message, public_key.clone());
            let decrypted_message = decrypt_message(encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        }

        #[test]
        fn encrypt_and_decrypt_long_message() {
            let (public_key, private_key) = generate_keys(1024).unwrap();
            let message = "This is a very long message that exceeds the chunk size.";

            let encrypted_message = encrypt_message(message, public_key.clone());
            let decrypted_message = decrypt_message(encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        }

        #[test]
        fn encrypt_and_decrypt_with_small_key() {
            let (public_key, private_key) = generate_keys(512).unwrap();
            let message = "This is a test message.";

            let encrypted_message = encrypt_message(message, public_key.clone());
            let decrypted_message = decrypt_message(encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        }

        #[test]
        fn encrypt_and_decrypt_with_large_key() {
            let (public_key, private_key) = generate_keys(2048).unwrap();
            let message = "This is a test message.";

            let encrypted_message = encrypt_message(message, public_key.clone());
            let decrypted_message = decrypt_message(encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        }

        #[test]
        fn encrypt_and_decrypt_very_long_message() {
            let (public_key, private_key) = generate_keys(1024).unwrap();
            let message = "This is a very long message. It is so long that it exceeds the chunk size many times over. \
                   In fact, it is so long that it might even be considered a small book or a short novel. \
                   It contains many characters, words, sentences, and paragraphs, and it goes on and on and on. \
                   But despite its length, it is still just a single message, and it should be encrypted and decrypted correctly.";

            let encrypted_message = encrypt_message(message, public_key.clone());
            let decrypted_message = decrypt_message(encrypted_message, private_key);

            assert_eq!(message, decrypted_message);
        }
    }
}
