use sop_kode::caesar_shift;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_shift_encrypts_correctly() {
        let plaintext = "HELLO";
        let alphabet = vec![
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "KHOOR");
    }

    #[test]
    fn caesar_shift_handles_non_alphabet_characters() {
        let plaintext = "HELLO WORLD";
        let alphabet = vec![
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "KHOORZRUOG");
    }

    #[test]
    fn caesar_shift_handles_empty_string() {
        let plaintext = "";
        let alphabet = vec![
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let encrypted = caesar_shift(plaintext, 3, alphabet);
        assert_eq!(encrypted, "");
    }

    #[test]
    fn caesar_shift_handles_zero_shift() {
        let plaintext = "HELLO";
        let alphabet = vec![
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let encrypted = caesar_shift(plaintext, 0, alphabet);
        assert_eq!(encrypted, "HELLO");
    }
}
