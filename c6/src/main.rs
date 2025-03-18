//! A simple timing game where players try to estimate a 10-second interval.
//!
//! The game prompts users to press Enter twice: once to start the timer and
//! once when they think 10 seconds have elapsed. It then provides feedback on
//! their timing accuracy.
use std::io::BufRead;

fn main() {
    println!("This is a game that tests how good you are at guessing if 10 seconds has elapsed.");
    println!("Press Enter to start the game.");
    println!("Press Enter again when you think exactly 10 seconds has elapsed.");

    let mut reader = std::io::BufReader::new(std::io::stdin());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    println!("Start!");
    let start_time = std::time::Instant::now();

    reader.read_line(&mut input).unwrap();
    println!("Stop!");
    let elapsed_time = start_time.elapsed();

    if elapsed_time.as_secs() >= 10 {
        println!(
            "You waited too long! You waited for {} seconds.",
            elapsed_time.as_secs()
        );
    } else {
        println!(
            "You didn't wait long enough! You only waited for {} seconds.",
            elapsed_time.as_secs()
        );
    }
}
