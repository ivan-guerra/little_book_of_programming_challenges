//! # ASCII Caesar Cipher
//!
//! This module implements a simple interactive ASCII Caesar cipher
//! that encrypts and decrypts text by shifting characters.
//!
//! ## Features
//!
//! - **Bidirectional Operation**: Supports both encryption and decryption
//! - **ASCII Support**: Works with the full ASCII character set (0-127)
//! - **Character Shifting**: Shifts characters by a user-specified value
//! - **Wraparound Handling**: Properly handles shifts that exceed ASCII bounds
//! - **Non-ASCII Preservation**: Leaves non-ASCII characters unchanged
//! - **Input Validation**: Provides clear feedback for invalid inputs
use std::fmt::{self, Display, Formatter};

enum CipherMode {
    Encrypt,
    Decrypt,
}

impl Display for CipherMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CipherMode::Encrypt => "encrypt",
                CipherMode::Decrypt => "decrypt",
            }
        )
    }
}

fn prompt_for_cipher_mode() -> CipherMode {
    loop {
        println!("Enter 'e' to encrypt or 'd' to decrypt: ");
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim() {
            "e" => return CipherMode::Encrypt,
            "d" => return CipherMode::Decrypt,
            _ => println!("Invalid input. Please enter 'e' or 'd'."),
        }
    }
}

fn prompt_for_shift_value() -> i32 {
    loop {
        println!("Enter the shift value: ");
        let mut shift = String::new();
        std::io::stdin().read_line(&mut shift).unwrap();

        match shift.trim().parse() {
            Ok(num) => return num,
            Err(e) => eprintln!(
                "Error: {}. Please enter a valid number in the range 0 to 255.",
                e
            ),
        }
    }
}

fn prompt_for_text() -> String {
    loop {
        println!("Enter the text: ");
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }
        return input.trim().to_string();
    }
}

fn apply_cipher(text: &str, shift: i32) -> String {
    text.chars().map(|c| shift_char(c, shift)).collect()
}

fn shift_char(c: char, shift: i32) -> char {
    if !c.is_ascii() {
        return c;
    }

    const ASCII_ALPHABET_LEN: i32 = 128;
    let pos = c as i32;
    let shifted = (pos + shift).rem_euclid(ASCII_ALPHABET_LEN);

    char::from_u32(shifted as u32).unwrap_or(c)
}

fn main() {
    let mode = prompt_for_cipher_mode();
    let text = prompt_for_text();
    let shift = prompt_for_shift_value();
    println!(
        "{}ion result: {}",
        mode,
        match mode {
            CipherMode::Encrypt => apply_cipher(&text, shift),
            CipherMode::Decrypt => apply_cipher(&text, -shift),
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_char_correctly_shifts_ascii_characters() {
        assert_eq!(shift_char('a', 1), 'b');
        assert_eq!(shift_char('z', 1), '{');
        assert_eq!(shift_char('A', 1), 'B');
    }

    #[test]
    fn shift_char_wraps_around_when_exceeding_ascii_range() {
        assert_eq!(shift_char('~', 1), '\u{7f}');
        assert_eq!(shift_char('\u{7f}', 1), '\u{00}');
    }

    #[test]
    fn shift_char_handles_negative_shifts() {
        assert_eq!(shift_char('b', -1), 'a');
        assert_eq!(shift_char('a', -1), '`');
    }

    #[test]
    fn shift_char_preserves_non_ascii_characters() {
        assert_eq!(shift_char('é', 5), 'é');
        assert_eq!(shift_char('ñ', -10), 'ñ');
        assert_eq!(shift_char('日', 20), '日');
    }

    #[test]
    fn shift_char_wraps_correctly_with_large_shifts() {
        assert_eq!(shift_char('a', 128), 'a'); // Full cycle
        assert_eq!(shift_char('a', 129), 'b'); // Full cycle plus one
        assert_eq!(shift_char('a', -128), 'a'); // Negative full cycle
    }

    #[test]
    fn apply_cipher_correctly_shifts_all_characters_in_string() {
        assert_eq!(apply_cipher("abc", 1), "bcd");
        assert_eq!(apply_cipher("xyz", 1), "yz{");
    }

    #[test]
    fn apply_cipher_handles_empty_strings() {
        assert_eq!(apply_cipher("", 5), "");
    }

    #[test]
    fn apply_cipher_preserves_non_ascii_characters_in_string() {
        assert_eq!(apply_cipher("café", 1), "dbgé");
    }

    #[test]
    fn apply_cipher_properly_handles_negative_shifts() {
        assert_eq!(apply_cipher("bcd", -1), "abc");
    }

    #[test]
    fn apply_cipher_correctly_processes_strings_with_spaces_and_symbols() {
        assert_eq!(apply_cipher("Hello, World!", 1), "Ifmmp-!Xpsme\"");
    }
}
