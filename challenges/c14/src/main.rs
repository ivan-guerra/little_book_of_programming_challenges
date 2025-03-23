//! # Higher or Lower Game
//!
//! This module implements a number guessing game where the player predicts if the next
//! randomly generated number will be higher or lower than the current one.
//!
//! ## Game Rules
//!
//! - The game generates random numbers between 1 and 13
//! - Players must guess if the next number will be higher or lower than the current number
//! - Players need to guess correctly 10 times in a row to win
//! - Players have 2 lives (attempts) to achieve the winning streak
//!
//! ## Features
//!
//! - Interactive command-line interface
//! - Random number generation for unpredictable gameplay
//! - Input validation to ensure valid guesses
//! - Multiple lives system for replayability
//! - Streak-based win condition to test player prediction skills
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Guess {
    Higher,
    Lower,
}

fn get_rand_num(min: u64, max: u64) -> u64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

fn prompt_for_guess() -> Guess {
    println!("Higher(H) or Lower(L)?");

    let mut input = String::new();
    loop {
        input.clear();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim() {
            "H" => return Guess::Higher,
            "L" => return Guess::Lower,
            _ => {
                println!("Invalid input. Please enter 'H' for higher or 'L' for lower.");
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

fn main() {
    println!("You will be presented with a random number between 1 and 13.");
    println!("You must guess if the next number will be higher or lower.");
    println!("You must guess correctly 10 times in a row to win.");

    wait_on_enter();

    const LIMITS: (u64, u64) = (1, 13);
    const MAX_LIVES: u64 = 2;
    const WINNING_SCORE: u64 = 10;

    let mut correct_guesses = 0;
    let mut prev_num = get_rand_num(LIMITS.0, LIMITS.1);
    for lives in 0..MAX_LIVES {
        for _ in 0..LIMITS.1 {
            println!("Starting number: {}", prev_num);
            let guess = prompt_for_guess();
            let num = get_rand_num(LIMITS.0, LIMITS.1);

            if (num > prev_num && guess == Guess::Higher)
                || (num < prev_num && guess == Guess::Lower)
            {
                correct_guesses += 1;
            }
            prev_num = num;
        }

        if correct_guesses >= WINNING_SCORE {
            break;
        } else if lives < MAX_LIVES - 1 {
            println!(
                "Sorry, you lost. You have {} lives remaining.",
                MAX_LIVES - lives - 1
            );
            wait_on_enter();
            correct_guesses = 0;
        }
    }

    if correct_guesses >= WINNING_SCORE {
        println!("Congratulations! You won!");
    } else {
        println!("Sorry, you lost. Better luck next time!");
    }
}
