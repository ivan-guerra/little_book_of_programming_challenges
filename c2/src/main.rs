//! A simple command-line name greeting program.
//!
//! This program demonstrates basic Rust I/O operations by:
//! - Prompting the user for their name
//! - Reading input from stdin
//! - Trimming whitespace from the input
//! - Printing a personalized greeting
//!
//! The program uses proper error handling with Result types and
//! efficient I/O operations with buffered input/output.
fn main() -> Result<(), std::io::Error> {
    println!("What is you name?");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;

    println!("Hello, {}", name.trim());

    Ok(())
}
