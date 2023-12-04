/// Performs a Caesar shift on a given string.
///
/// # Arguments
///
/// * `m` - A string slice that holds the string to be encrypted.
/// * `k` - An unsigned 8-bit integer that represents the shift value.
/// * `n` - A vector of string slices that represents the alphabet.
///
/// # Returns
///
/// * A String that represents the encrypted string.
///
/// # Example
///
/// ```
/// let plaintext = "METTE";
/// let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
///                     "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
///                     "U", "V", "W", "X", "Y", "Z", "Æ", "Ø", "Å"];
/// let encrypted = caesar_shift(plaintext, 3, alphabet);
/// println!("Encrypted: {}", encrypted); // PHWWH
/// ```
pub fn caesar_shift(m: &str, k: u8, n: Vec<&str>) -> String {
    m.chars()
        .filter(|&c| c != ' ')
        .map(|i| {
            let pos = n.iter().position(|&r| r == i.to_string()).unwrap();
            let new_pos = (pos + k as usize) % n.len();
            n[new_pos].to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_shift_encrypts_correctly() {
        let plaintext = "HELLO";
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                            "U", "V", "W", "X", "Y", "Z"];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "KHOOR");
    }

    #[test]
    fn caesar_shift_handles_non_alphabet_characters() {
        let plaintext = "HELLO WORLD";
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                            "U", "V", "W", "X", "Y", "Z"];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "KHOORZRUOG");
    }

    #[test]
    fn caesar_shift_handles_empty_string() {
        let plaintext = "";
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                            "U", "V", "W", "X", "Y", "Z"];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "");
    }

    #[test]
    fn caesar_shift_handles_zero_shift() {
        let plaintext = "HELLO";
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                            "U", "V", "W", "X", "Y", "Z"];
        let encrypted = caesar_shift(plaintext, 0, alphabet);
        assert_eq!(encrypted, "HELLO");
    }
}