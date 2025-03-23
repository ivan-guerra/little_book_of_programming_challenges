//! # Subtraction Game
//!
//! This module implements a simple turn-based number subtraction game where
//! a player competes against a computer opponent.
//!
//! ## Game Rules
//!
//! - The game starts with a random number between 20 and 30
//! - Players take turns subtracting 1-3 from the current number
//! - The player who reduces the number to exactly 0 loses
//! - The computer uses a simple strategy for numbers 1-3 and random moves otherwise
//!
//! ## Features
//!
//! - Interactive command-line interface
//! - Simple AI opponent with basic strategy for end-game situations
//! - Random starting position for varied gameplay
//! - Input validation to ensure legal moves
//! - Clear feedback after each move
use rand::Rng;

fn get_rand_num(min: u64, max: u64) -> u64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

fn prompt_for_number(limits: (u64, u64)) -> u64 {
    println!("How many do you want to remove? ");

    let mut input = String::new();
    loop {
        input.clear();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().parse() {
            Ok(num) => {
                if num < limits.0 || num > limits.1 {
                    println!(
                        "Invalid input. Please enter a number between {} and {}.",
                        limits.0, limits.1
                    );
                    continue;
                }
                return num;
            }
            Err(e) => {
                eprintln!(
                    "Error: {}. Please enter a number between {} and {}.",
                    e, limits.0, limits.1
                );
                continue;
            }
        }
    }
}

fn make_move_ai(num: u64) -> u64 {
    match num {
        1 => 1,
        2 => 1,
        3 => 2,
        _ => get_rand_num(1, 3),
    }
}

fn main() {
    println!("In this game, you are presented with a random starting number.");
    println!("Each round, you must chose a number in the range 1-3 to subtract from the starting number.");
    println!("The player who reaches 0 is the loser.");
    println!("Press Enter to start the game.");

    if let Err(e) = std::io::stdin().read_line(&mut String::new()) {
        eprintln!("Error: {}", e);
        return;
    }

    const LIMITS: (u64, u64) = (1, 3);
    let mut num = get_rand_num(20, 30);
    let mut deduction: u64;
    let mut is_player_turn = true;
    loop {
        println!("The current number is: {}", num);
        if is_player_turn {
            deduction = prompt_for_number(LIMITS);
            println!("Player removed: {}", deduction);
        } else {
            deduction = make_move_ai(num);
            println!("Computer removed: {}", deduction);
        }

        num = num.saturating_sub(deduction);
        println!("{} left.", num);

        if num == 0 {
            if is_player_turn {
                println!("You lost!");
            } else {
                println!("You won!");
            }
            break;
        }

        is_player_turn = !is_player_turn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_move_ai_returns_1_when_number_is_1() {
        assert_eq!(make_move_ai(1), 1);
    }

    #[test]
    fn make_move_ai_returns_1_when_number_is_2() {
        assert_eq!(make_move_ai(2), 1);
    }

    #[test]
    fn make_move_ai_returns_2_when_number_is_3() {
        assert_eq!(make_move_ai(3), 2);
    }

    #[test]
    fn make_move_ai_returns_number_in_range_for_larger_inputs() {
        // Test several larger numbers to ensure the output is always in range
        for i in 4..20 {
            let result = make_move_ai(i);
            assert!(
                (1..=3).contains(&result),
                "Expected move to be between 1 and 3, got {}",
                result
            );
        }
    }
}
