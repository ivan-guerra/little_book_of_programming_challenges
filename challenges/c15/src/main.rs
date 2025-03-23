//! # Text Query Tool
//!
//! This module implements a simple interactive text processing utility that allows
//! users to perform common text analysis operations on input sentences.
//!
//! ## Features
//!
//! - Interactive command-line interface
//! - Multiple query modes:
//!   - Word counting - counts the number of words in a sentence
//!   - Text reversal - reverses the characters in a sentence
//! - Input validation with clear error handling
//! - Simple user interface with clear prompts and feedback
enum QueryType {
    Count,
    Reverse,
}

fn prompt_for_query_type() -> QueryType {
    println!("Would you like to count words (C) or reverse your sentence (R)?");
    let mut input = String::new();
    loop {
        input.clear();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }
        match input.trim().to_uppercase().as_str() {
            "C" => return QueryType::Count,
            "R" => return QueryType::Reverse,
            _ => {
                println!("Invalid input. Please enter 'C' or 'R'.");
                continue;
            }
        }
    }
}

fn main() {
    let query = prompt_for_query_type();

    println!("Enter your sentence: ");
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("Error: {}", e);
        return;
    }

    match query {
        QueryType::Count => {
            println!("Word count: {}", input.split_whitespace().count());
        }
        QueryType::Reverse => {
            println!(
                "Reversed sentence: {}",
                input.trim().chars().rev().collect::<String>()
            );
        }
    }
}
