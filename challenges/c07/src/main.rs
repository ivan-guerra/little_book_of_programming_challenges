//! # Alphabet Typing Speed Game
//!
//! This module implements a simple game that measures how quickly you can type the entire
//! alphabet correctly. The program tracks your best time across multiple attempts.
//!
//! ## Features
//!
//! - Measures typing speed for the complete alphabet
//! - Validates input to ensure the entire alphabet is typed correctly
//! - Tracks best performance across multiple attempts
//! - Handles various input formats including mixed case and whitespace
//!
//! ## How to Play
//!
//! 1. Run the program and press Enter to start
//! 2. Type the complete alphabet (a-z) as quickly as possible
//! 3. Press Enter to submit your attempt
//! 4. The program will show your time if successful, or prompt you to try again
//! 5. Press Enter to play again or 'q' to quit and see your best time
fn is_valid_alphabet(input: &str) -> bool {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let input = input.trim().to_lowercase();
    input == ALPHABET
}

fn main() {
    println!("This is a game to see how fast you can type the alphabet.");
    println!("Press Enter to start the game.");

    let _ = std::io::stdin().read_line(&mut String::new());
    let mut best_time: f64 = f64::INFINITY;
    loop {
        println!("Start typing, press enter to submit!");
        let start_time = std::time::Instant::now();

        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            break;
        }

        let elapsed_time = start_time.elapsed();
        if is_valid_alphabet(&input) {
            println!(
                "You typed the alphabet in {:.2} seconds!",
                elapsed_time.as_secs_f64()
            );
            best_time = best_time.min(elapsed_time.as_secs_f64());
        } else {
            println!("You didn't type the alphabet correctly. Try again!");
        }

        println!("Press Enter to play again or 'q' to quit.");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            break;
        }
        if input.trim() == "q" {
            break;
        }
    }

    if best_time != f64::INFINITY {
        println!("Your best time was {:.2} seconds!", best_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_alphabet_accepts_correct_alphabet() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert!(is_valid_alphabet(input), "Should accept correct alphabet");
    }

    #[test]
    fn is_valid_alphabet_accepts_with_whitespace() {
        let input = "abcdefghijklmnopqrstuvwxyz\n";
        assert!(
            is_valid_alphabet(input),
            "Should accept alphabet with trailing whitespace"
        );
    }

    #[test]
    fn is_valid_alphabet_accepts_uppercase() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert!(is_valid_alphabet(input), "Should accept uppercase alphabet");
    }

    #[test]
    fn is_valid_alphabet_accepts_mixed_case() {
        let input = "AbCdEfGhIjKlMnOpQrStUvWxYz";
        assert!(
            is_valid_alphabet(input),
            "Should accept mixed case alphabet"
        );
    }

    #[test]
    fn is_valid_alphabet_rejects_incorrect_order() {
        let input = "abcdefghijklmnopqrstuvwzyx";
        assert!(!is_valid_alphabet(input), "Should reject incorrect order");
    }

    #[test]
    fn is_valid_alphabet_rejects_missing_letters() {
        let input = "abcdefghijklmnopqrstuvwxy";
        assert!(!is_valid_alphabet(input), "Should reject missing letters");
    }

    #[test]
    fn is_valid_alphabet_rejects_duplicate_letters() {
        let input = "abcdefghijklmnopqrstuvwxyzz";
        assert!(!is_valid_alphabet(input), "Should reject duplicate letters");
    }

    #[test]
    fn is_valid_alphabet_rejects_extra_characters() {
        let input = "abcdefghijklmnopqrstuvwxyz123";
        assert!(!is_valid_alphabet(input), "Should reject extra characters");
    }

    #[test]
    fn is_valid_alphabet_rejects_empty_string() {
        let input = "";
        assert!(!is_valid_alphabet(input), "Should reject empty string");
    }
}
