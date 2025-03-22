//! # Rock, Paper, Scissors Game
//!
//! This module implements a simple interactive Rock, Paper, Scissors game.
//! It allows players to make moves against a computer opponent and tracks
//! win/loss/tie results.
//!
//! ## Features
//!
//! - Interactive gameplay with keyboard input
//! - Random computer move generation
//! - Game state tracking (win, lose, tie)
//! - Case-insensitive input handling
//! - Clear game result feedback
//!
//! The implementation follows standard Rock-Paper-Scissors rules where:
//! Rock beats Scissors, Paper beats Rock, and Scissors beats Paper.
use rand::seq::IndexedRandom;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_move_from_input(input: &str) -> Option<Move> {
    match input.trim().to_lowercase().as_str() {
        "rock" => Some(Move::Rock),
        "paper" => Some(Move::Paper),
        "scissors" => Some(Move::Scissors),
        _ => None,
    }
}

fn player_wins(player: &Move, computer: &Move) -> bool {
    matches!(
        (player, computer),
        (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper)
    )
}

fn get_rand_move() -> Move {
    static MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];
    MOVES
        .choose(&mut rand::rng())
        .copied()
        .unwrap_or(Move::Rock)
}

fn main() {
    println!("Play a game of Rock, Paper, Scissors. Press ENTER to begin.");

    if let Err(e) = std::io::stdin().read_line(&mut String::new()) {
        eprintln!("Failed to read line: {}", e);
        return;
    }

    let mut input = String::new();
    loop {
        println!("Enter your move (rock, paper, or scissors): ");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", e);
            return;
        }

        let player_move = match get_move_from_input(&input) {
            Some(m) => m,
            None => {
                println!("Invalid move. Please try again.");
                input.clear();
                continue;
            }
        };
        let computer_move = get_rand_move();

        if player_wins(&player_move, &computer_move) {
            println!(
                "You win! You chose {:?} and the computer chose {:?}.",
                player_move, computer_move
            );
        } else if player_move == computer_move {
            println!("It's a tie! You both chose {:?}.", player_move);
        } else {
            println!(
                "You lose! You chose {:?} and the computer chose {:?}.",
                player_move, computer_move
            );
        }

        input.clear();
        println!("Press ENTER to play again or type 'q' to quit.");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Failed to read line: {}", e);
            return;
        }

        if input.trim() == "q" {
            break;
        }
        input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_move_from_input_returns_rock_for_rock_input() {
        assert_eq!(get_move_from_input("rock"), Some(Move::Rock));
        assert_eq!(get_move_from_input("Rock"), Some(Move::Rock));
        assert_eq!(get_move_from_input("ROCK"), Some(Move::Rock));
        assert_eq!(get_move_from_input("rock "), Some(Move::Rock));
        assert_eq!(get_move_from_input(" rock"), Some(Move::Rock));
    }

    #[test]
    fn get_move_from_input_returns_paper_for_paper_input() {
        assert_eq!(get_move_from_input("paper"), Some(Move::Paper));
        assert_eq!(get_move_from_input("Paper"), Some(Move::Paper));
        assert_eq!(get_move_from_input("PAPER"), Some(Move::Paper));
        assert_eq!(get_move_from_input("paper "), Some(Move::Paper));
        assert_eq!(get_move_from_input(" paper"), Some(Move::Paper));
    }

    #[test]
    fn get_move_from_input_returns_scissors_for_scissors_input() {
        assert_eq!(get_move_from_input("scissors"), Some(Move::Scissors));
        assert_eq!(get_move_from_input("Scissors"), Some(Move::Scissors));
        assert_eq!(get_move_from_input("SCISSORS"), Some(Move::Scissors));
        assert_eq!(get_move_from_input("scissors "), Some(Move::Scissors));
        assert_eq!(get_move_from_input(" scissors"), Some(Move::Scissors));
    }

    #[test]
    fn get_move_from_input_returns_none_for_invalid_input() {
        assert_eq!(get_move_from_input(""), None);
        assert_eq!(get_move_from_input("invalid"), None);
        assert_eq!(get_move_from_input("123"), None);
        assert_eq!(get_move_from_input("scissor"), None);
    }

    #[test]
    fn player_wins_returns_true_when_player_wins() {
        assert!(player_wins(&Move::Rock, &Move::Scissors));
        assert!(player_wins(&Move::Paper, &Move::Rock));
        assert!(player_wins(&Move::Scissors, &Move::Paper));
    }

    #[test]
    fn player_wins_returns_false_for_same_moves() {
        assert!(!player_wins(&Move::Rock, &Move::Rock));
        assert!(!player_wins(&Move::Paper, &Move::Paper));
        assert!(!player_wins(&Move::Scissors, &Move::Scissors));
    }

    #[test]
    fn player_wins_returns_false_when_player_loses() {
        assert!(!player_wins(&Move::Scissors, &Move::Rock));
        assert!(!player_wins(&Move::Rock, &Move::Paper));
        assert!(!player_wins(&Move::Paper, &Move::Scissors));
    }
}
