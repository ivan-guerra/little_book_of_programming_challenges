//! # Name Duplicate Counter
//!
//! This module implements a simple interactive name duplicate counter
//! that tracks and reports repeated names entered by the user.
//!
//! ## Features
//!
//! - **Data Collection**: Gathers names interactively until user decides to exit
//! - **Duplicate Detection**: Identifies and counts repeated name entries
//! - **Hash-based Storage**: Uses efficient HashMap for name frequency tracking
//! - **Error Handling**: Provides clear feedback for input errors
//! - **Filtered Reporting**: Only displays names that appear multiple times
//! - **Interactive Interface**: Allows continuous input with a clear exit command
use std::collections::HashMap;

fn prompt_for_names() -> HashMap<String, u32> {
    const EXIT_MARKER: &str = "exit";
    let mut names = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter a name (or 'exit' to finish): ");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if input.trim() == EXIT_MARKER {
            break;
        }

        let count = names.entry(input.trim().to_string()).or_insert(0);
        *count += 1;
    }
    names
}

fn main() {
    let names = prompt_for_names();
    names
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .for_each(|(name, count)| {
            println!("{} has {} duplicates.", name, count);
        });
}
