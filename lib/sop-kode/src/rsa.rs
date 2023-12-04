use std::collections::HashMap;

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
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
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
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extended_gcd(a, m);
    (x % m + m) % m
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
        acc + digit * base.pow(i as u32)
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
pub fn chunk_message(chars: Vec<char>, chunk_size: usize, alphabet_positions: &HashMap<char, i64>) -> Vec<Vec<i64>> {
    let nums: Vec<i64> = chars.into_iter().map(|c| alphabet_positions[&c]).collect();

    let chunks: Vec<Vec<i64>> = nums.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;


    #[test]
    fn extended_gcd_returns_correct_values() {
        assert_eq!(extended_gcd(3, 11), (1, 4, -1));
        assert_eq!(extended_gcd(10, 17), (1, -5, 3));
    }


    #[test]
    fn mod_inverse_returns_correct_values() {
        assert_eq!(mod_inverse(3, 11), 4);
        assert_eq!(mod_inverse(10, 17), 12);
    }

    #[test]
    fn base_n_to_base10_returns_correct_values() {
        assert_eq!(base_n_to_base10(&vec![1, 0, 1], 2), 5);
        assert_eq!(base_n_to_base10(&vec![1, 2, 3], 10), 123);
        assert_eq!(base_n_to_base10(&vec![1, 2, 3], 16), 291);
        assert_eq!(base_n_to_base10(&vec![1, 0], 28), 28);
    }

    #[test]
    fn chunk_message_returns_correct_chunks() {
        let alphabet_positions: HashMap<char, i64> = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ".chars().enumerate().map(|(i, c)| (c, i as i64)).collect();
        assert_eq!(chunk_message("HELLO".chars().collect(), 2, &alphabet_positions), vec![vec![7, 4], vec![11, 11], vec![14]]);
    }
}
