//! A simple joke-telling program that presents random jokes with colorful answers.
//!
//! This program selects a random joke from a predefined collection, displays the
//! question, waits for user input (Enter key), and then shows the answer in a
//! random color.
//!
//! The jokes are child-friendly and sourced from an educational [blog](https://childrenlearningenglishaffectively.blogspot.com/2013/05/50-easy-jokes-for-young-english-learners.html) for
//! English learners.
use colored::Colorize;
use once_cell::sync::Lazy;
use rand::seq::IndexedRandom;
use rand::Rng;
use std::collections::HashMap;
use std::io::Write;

static JOKES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "Why won’t the elephant use the computer?",
        "He’s afraid of the mouse!",
    );
    m.insert(
        "Which are the stronger days of the week?",
        "Saturday and Sunday. The rest are weekdays.",
    );
    m.insert(
        "Which runs faster, hot or cold?",
        "Hot. Everyone can catch a cold.",
    );
    m.insert(
        "What did the math book tell the pencil?",
        "I have a lot of problems.",
    );
    m.insert("Where can you find an ocean without water?", "on a map!");
    m.insert(
        "Why do fish swim in salt water?",
        "Pepper makes them sneeze.",
    );
    m.insert("What is a robot’s favorite snack?", "Computer chips!");
    m.insert(
        "How did the soldier fit his tank in his house?",
        "It was a fish tank!",
    );
    m.insert("Why did the computer go to the doctors?", "It had a virus.");
    m.insert(
        "Why did the man throw a clock out the window?",
        "He wanted time to fly.",
    );
    m.insert("Where do cows go on dates?", "MOOOOvies");
    m.insert(
        "What kind of snack do you have during a scary movie?",
        "I scream (ice cream)",
    );
    m.insert("How can you tell the ocean is friendly?", "It waves!");
    m.insert("How do small children travel?", "In mini-vans");
    m.insert("What has  wheels and flies?", "a garbage truck!");
    m.insert(
        "Why didn’t the skeleton go to the party?",
        "He had NO BODY to go with.",
    );
    m.insert(
        "What kind of witch likes the beach?",
        "a SAND witch (sandwich)!",
    );
    m.insert("What kind of key does not open a lock?", "a mon – KEY!");
    m.insert("What always falls and never gets hurt?", "rain!");
    m.insert(
        "What letters are not in the alphabet?",
        "The ones in the mail.",
    );
    m.insert(
        "Why did the boy throw the butter out the window?",
        "to see a butterfly!",
    );
    m.insert(
        "What room is a dead man most afraid of?",
        "The living room!",
    );
    m.insert(
        "What did one wall say to the other?",
        "Hey, let’s meet in the corner.",
    );
    m.insert(
        "Why do birds fly south in the winter?",
        "Because it’s too far to walk!",
    );
    m.insert("Why is six afraid of seven?", "Because 7 ATE 9");
    m
});

fn get_random_color() -> colored::Color {
    use colored::Color;

    static COLORS: [colored::Color; 14] = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];
    let mut rng = rand::rng();
    *COLORS.choose(&mut rng).unwrap_or(&Color::White)
}

fn main() -> std::io::Result<()> {
    let jokes: Vec<_> = JOKES.iter().collect();
    let (question, answer) = jokes[rand::rng().random_range(0..jokes.len())];
    print!("{question} (press enter) ");
    // We flush to ensure the message gets printed immediately.
    std::io::stdout().flush()?;

    // Wait for the user to press enter.
    let _ = std::io::stdin().read_line(&mut String::new())?;

    println!("{}", answer.color(get_random_color()));

    Ok(())
}
