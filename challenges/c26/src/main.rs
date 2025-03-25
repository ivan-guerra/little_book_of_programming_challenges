//! # Mastermind Guessing Game
//!
//! This module implements an interactive Mastermind-style code-breaking game
//! where players guess a randomly generated numeric code.
//!
//! ## Features
//!
//! - **Random Code Generation**: Creates random numeric codes of configurable length
//! - **Feedback System**: Provides feedback on correct digits and positions after each guess
//! - **Input Validation**: Ensures guesses are valid numeric sequences of the correct length
//! - **Error Handling**: Provides clear feedback for invalid inputs
//! - **Game Logic**: Tracks game progress and determines win conditions
//! - **Limited Attempts**: Enforces a maximum number of guesses before game over
use rand::Rng;
use std::collections::HashMap;

struct GuessStats {
    correct_digits: u32,
    correct_positions: u32,
}

fn evaluate_guess(guess: &str, target: &str) -> GuessStats {
    // Pass 1: Count correct positions
    let correct_positions =
        guess
            .chars()
            .zip(target.chars())
            .fold(0, |acc, (g, t)| if g == t { acc + 1 } else { acc });

    // Pass 2: Count the number of correct digits regardless of position
    let guess_counts = guess.chars().fold(HashMap::new(), |mut counts, c| {
        *counts.entry(c).or_insert(0) += 1;
        counts
    });
    let target_counts = target.chars().fold(HashMap::new(), |mut counts, c| {
        *counts.entry(c).or_insert(0) += 1;
        counts
    });
    let mut correct_digits = 0;
    for (c, gcount) in guess_counts {
        if target_counts.contains_key(&c) {
            let tcount = target_counts[&c];
            correct_digits += match gcount.cmp(&tcount) {
                std::cmp::Ordering::Less => gcount,
                _ => tcount,
            };
        }
    }

    GuessStats {
        correct_digits,
        correct_positions,
    }
}

fn generate_code(num_digits: u32) -> String {
    (0..num_digits)
        .map(|_| rand::rng().random_range(0..10).to_string())
        .collect()
}

fn prompt_user_for_guess(num_digits: u32) -> String {
    loop {
        println!("Enter a {}-digit guess: ", num_digits);
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let has_invalid_digit_count = input.trim().len() != num_digits as usize;
        let has_non_numeric_chars = !input.trim().chars().all(char::is_numeric);
        if has_invalid_digit_count || has_non_numeric_chars {
            println!("Invalid input. Please enter a {}-digit number.", num_digits);
            continue;
        } else {
            return input.trim().to_string();
        }
    }
}

fn main() {
    const CODE_LENGTH: u32 = 4;
    const MAX_GUESSES: u32 = 12;

    let target = generate_code(CODE_LENGTH);
    for _ in 0..MAX_GUESSES {
        let guess = prompt_user_for_guess(CODE_LENGTH);
        let stats = evaluate_guess(&guess, &target);
        if stats.correct_positions == CODE_LENGTH {
            println!("Congratulations! You've guessed the code.");
            break;
        } else {
            println!(
                "Correct digits: {}, correct positions: {}",
                stats.correct_digits, stats.correct_positions
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_guess_returns_zero_when_no_matching_digits() {
        let stats = evaluate_guess("1234", "5678");
        assert_eq!(stats.correct_digits, 0);
        assert_eq!(stats.correct_positions, 0);
    }

    #[test]
    fn evaluate_guess_counts_correct_digits_in_wrong_positions() {
        let stats = evaluate_guess("1234", "4321");
        assert_eq!(stats.correct_digits, 4);
        assert_eq!(stats.correct_positions, 0);
    }

    #[test]
    fn evaluate_guess_counts_correct_digits_in_correct_positions() {
        let stats = evaluate_guess("1234", "1256");
        assert_eq!(stats.correct_digits, 2);
        assert_eq!(stats.correct_positions, 2);
    }

    #[test]
    fn evaluate_guess_handles_mixed_correct_and_incorrect_positions() {
        let stats = evaluate_guess("1234", "1432");
        assert_eq!(stats.correct_digits, 4);
        assert_eq!(stats.correct_positions, 2);
    }

    #[test]
    fn evaluate_guess_handles_duplicate_digits_in_guess() {
        let stats = evaluate_guess("1122", "1234");
        assert_eq!(stats.correct_digits, 2);
        assert_eq!(stats.correct_positions, 1);
    }

    #[test]
    fn evaluate_guess_handles_duplicate_digits_in_target() {
        let stats = evaluate_guess("1234", "1122");
        assert_eq!(stats.correct_digits, 2);
        assert_eq!(stats.correct_positions, 1);
    }

    #[test]
    fn evaluate_guess_identifies_perfect_match() {
        let stats = evaluate_guess("1234", "1234");
        assert_eq!(stats.correct_digits, 4);
        assert_eq!(stats.correct_positions, 4);
    }

    #[test]
    fn evaluate_guess_handles_empty_strings() {
        let stats = evaluate_guess("", "");
        assert_eq!(stats.correct_digits, 0);
        assert_eq!(stats.correct_positions, 0);
    }
}
