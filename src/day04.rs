use std::collections::HashSet;


fn num_of_valid_passphrases<F>(passphrases: &str, passphrase_validator: F) -> usize
where
    F: Fn(&str) -> bool
{
    passphrases
        .split("\n")
        .filter(|passphrase| { passphrase_validator(passphrase) }) //deref trait is automatically dereferencing here?
        .count()
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    passphrase.split_whitespace().count() == HashSet::<_>::from_iter(passphrase.split_whitespace()).len()
}

fn is_valid_advanced_passphrase(passphrase: &str) -> bool {
    let num_of_words = passphrase.split_whitespace().count();

    let sorted_words = passphrase
        .split_whitespace()
        .map(|word| { ordered_word(word) });

    let num_of_unique_sorted_words = HashSet::<_>::from_iter(sorted_words).len();

    num_of_words == num_of_unique_sorted_words
}

fn ordered_word(word: &str) -> Vec<char> {
    let mut chars = word.chars().collect::<Vec<_>>();
    chars.sort();
    chars
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_detect_valid_passphrases() {
        assert_eq!(is_valid_passphrase("aa bb cc dd ee"), true);
        assert_eq!(is_valid_passphrase("aa bb cc dd aa"), false);
        assert_eq!(is_valid_passphrase("aa bb cc dd aaa"), true);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day04.txt").unwrap();
        assert_eq!(num_of_valid_passphrases(input.as_str(), is_valid_passphrase), 455);
    }

    #[test]
    fn can_detect_valid_advanced_passphrases() {
        assert_eq!(is_valid_advanced_passphrase("abcde fghij"), true);
        assert_eq!(is_valid_advanced_passphrase("abcde xyz ecdab"), false);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day04.txt").unwrap();
        assert_eq!(num_of_valid_passphrases(input.as_str(), is_valid_advanced_passphrase), 186);
    }
}