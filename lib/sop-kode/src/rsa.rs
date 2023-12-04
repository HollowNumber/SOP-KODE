use std::collections::HashMap;

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x, y) = extended_gcd(b % a, a);
    (g, y - (b / a) * x, x)
}

pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extended_gcd(a, m);
    (x % m + m) % m
}


pub fn base_n_to_base10(digits: &Vec<i64>, base: i64) -> i64 {
    digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| {
        acc + digit * base.pow(i as u32)
    })
}


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
    }

    #[test]
    fn chunk_message_returns_correct_chunks() {
        let alphabet_positions: HashMap<char, i64> = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ".chars().enumerate().map(|(i, c)| (c, i as i64)).collect();
        assert_eq!(chunk_message("HELLO".chars().collect(), 2, &alphabet_positions), vec![vec![7, 4], vec![11, 11], vec![14]]);
    }
}