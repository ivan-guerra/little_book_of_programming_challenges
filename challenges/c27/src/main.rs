//! # Hangman Word Guessing Game
//!
//! This module implements a simple interactive Hangman-style word guessing game
//! where players attempt to guess a hidden word one letter at a time.
//!
//! ## Features
//!
//! - **Secure Word Input**: Hides the target word during input using password masking
//! - **Letter-by-Letter Guessing**: Allows players to guess one letter at a time
//! - **Visual Feedback**: Shows partially completed word with placeholders for unguessed letters
//! - **Life System**: Implements a limited number of incorrect guesses before game over
//! - **Input Validation**: Ensures only valid alphabetic characters are accepted as guesses
//! - **Case Handling**: Converts all input to uppercase for consistent comparison
//! - **Win/Loss Detection**: Identifies when the player has won or lost the game
fn prompt_for_word() -> String {
    loop {
        println!("Player 1, enter a word: ");
        match rpassword::read_password() {
            Ok(word) => return word.trim().to_uppercase().to_string(),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn prompt_for_letter(num_lives: u32) -> char {
    loop {
        println!("You have {} lives left - Letter? ", num_lives);
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().chars().next() {
            Some(letter) if letter.is_alphabetic() => return letter.to_uppercase().next().unwrap(),
            _ => {
                println!("Invalid input. Please enter a single letter.");
                continue;
            }
        }
    }
}

fn update_player_word(target_word: &str, guess_letter: char, player_word: &mut String) {
    for (i, target_char) in target_word.chars().enumerate() {
        if target_char == guess_letter {
            player_word.replace_range(i..i + 1, &guess_letter.to_string());
        }
    }
}

fn main() {
    const NUM_LIVES: u32 = 5;

    let target_word = prompt_for_word();
    let mut player_word = "*".repeat(target_word.len());
    println!("Word to guess: {}", player_word);

    let mut lives = NUM_LIVES;
    while lives > 0 {
        let letter = prompt_for_letter(lives);
        if target_word.find(letter).is_none() {
            lives -= 1;
        } else {
            update_player_word(&target_word, letter, &mut player_word);
        }

        if player_word.find('*').is_none() {
            println!("Congratulations! You've guessed the word: {}", target_word);
            break;
        } else if lives == 0 {
            println!("You've run out of lives. The word was: {}", target_word);
            break;
        } else {
            println!("Word to guess: {}", player_word);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_player_word_replaces_single_matching_character() {
        let target = "HELLO";
        let mut player_word = "*****".to_string();
        update_player_word(target, 'L', &mut player_word);
        assert_eq!(player_word, "**LL*");
    }

    #[test]
    fn update_player_word_replaces_multiple_instances_of_matching_character() {
        let target = "BANANA";
        let mut player_word = "******".to_string();
        update_player_word(target, 'A', &mut player_word);
        assert_eq!(player_word, "*A*A*A");
    }

    #[test]
    fn update_player_word_makes_no_changes_for_non_matching_character() {
        let target = "HELLO";
        let mut player_word = "*****".to_string();
        update_player_word(target, 'Z', &mut player_word);
        assert_eq!(player_word, "*****");
    }

    #[test]
    fn update_player_word_preserves_previously_guessed_characters() {
        let target = "HELLO";
        let mut player_word = "*E***".to_string();
        update_player_word(target, 'L', &mut player_word);
        assert_eq!(player_word, "*ELL*");
    }

    #[test]
    fn update_player_word_handles_empty_strings() {
        let target = "";
        let mut player_word = "".to_string();
        update_player_word(target, 'A', &mut player_word);
        assert_eq!(player_word, "");
    }

    #[test]
    fn update_player_word_is_case_sensitive() {
        let target = "Hello";
        let mut player_word = "*****".to_string();
        update_player_word(target, 'h', &mut player_word);
        assert_eq!(player_word, "*****"); // 'h' doesn't match 'H'
    }
}
