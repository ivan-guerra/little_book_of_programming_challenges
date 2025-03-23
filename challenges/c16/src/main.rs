//! # Number Guessing Game
//!
//! This module implements a two-mode number guessing game where either the player
//! or the computer tries to guess a secret number.
//!
//! ## Game Modes
//!
//! - **Player as Guesser**: The computer chooses a random number between 1 and 100,
//!   and the player tries to guess it with feedback after each attempt.
//!
//! - **Computer as Guesser**: The player thinks of a number between 1 and 100,
//!   and the computer uses a binary search algorithm to find it based on
//!   the player's feedback.
//!
//! ## Features
//!
//! - Interactive command-line interface
//! - Mode selection at the beginning of the game
//! - Input validation for all user entries
//! - Efficient binary search algorithm for computer guessing
//! - Tracking of attempts until the correct number is guessed
//! - Clear feedback after each guess attempt
use rand::Rng;

const GUESS_RNG: (u64, u64) = (1, 100);

enum Guesser {
    Human,
    Computer,
}

enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

fn prompt_for_guesser() -> Guesser {
    loop {
        let mut input = String::new();

        println!("Do you want to be the guesser? (y/n)");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }
        match input.trim().to_lowercase().as_str() {
            "y" => return Guesser::Human,
            "n" => return Guesser::Computer,
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                continue;
            }
        }
    }
}

fn wait_on_enter() {
    println!("Press Enter to continue.");
    if let Err(e) = std::io::stdin().read_line(&mut String::new()) {
        eprintln!("Error: {}", e);
    }
}

fn prompt_human_for_guess() -> u64 {
    println!("Enter your guess: ");
    loop {
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }
        match input.trim().parse() {
            Ok(num) => {
                if num < GUESS_RNG.0 || num > GUESS_RNG.1 {
                    println!(
                        "Invalid input. Please enter a number between {} and {}.",
                        GUESS_RNG.0, GUESS_RNG.1
                    );
                    continue;
                }
                return num;
            }
            Err(e) => {
                eprintln!(
                    "Error: {}. Please enter a number between {} and {}.",
                    e, GUESS_RNG.0, GUESS_RNG.1
                );
            }
        }
    }
}

fn prompt_for_guess() -> GuessResult {
    println!("Was the guess too high(H), too low(L), or correct(C)?");

    loop {
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim() {
            "H" => return GuessResult::TooHigh,
            "L" => return GuessResult::TooLow,
            "C" => return GuessResult::Correct,
            _ => {
                println!("Invalid input. Please enter 'H' for higher, 'L' for lower, or 'C' for correct.");
            }
        }
    }
}

fn human_game_loop() {
    let num = rand::rng().random_range(GUESS_RNG.0..=GUESS_RNG.1);
    let mut num_attempts = 0;
    loop {
        num_attempts += 1;
        let guess = prompt_human_for_guess();
        match guess.cmp(&num) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("Got it!");
                break;
            }
        }
    }
    println!("It took you {} attempts to guess the number.", num_attempts);
}

fn computer_game_loop() {
    let mut left = GUESS_RNG.0;
    let mut right = GUESS_RNG.1;
    let mut num_attempts = 0;
    loop {
        let guess = (left + right) / 2;
        num_attempts += 1;
        println!("The computer guesses: {}", guess);
        match prompt_for_guess() {
            GuessResult::TooLow => {
                left = guess + 1;
            }
            GuessResult::TooHigh => {
                right = guess - 1;
            }
            GuessResult::Correct => {
                break;
            }
        }
    }
    println!(
        "It took the computer {} attempts to guess the number.",
        num_attempts
    );
}

fn main() {
    println!("This is a guessing gaming. A number is chosen between 1 and 100.");
    println!("The player must guess the number to win.");
    wait_on_enter();

    match prompt_for_guesser() {
        Guesser::Human => human_game_loop(),
        Guesser::Computer => computer_game_loop(),
    }
}
