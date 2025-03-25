//! # Sports Results Tracker
//!
//! This module implements an interactive sports results tracker
//! that allows users to add and search for match results.
//!
//! ## Features
//!
//! - **Data Collection**: Allows users to add match results with team names and scores
//! - **Search Functionality**: Enables searching for results by team name
//! - **Menu-driven Interface**: Provides a simple menu for operation selection
//! - **Error Handling**: Handles invalid inputs with clear error messages
//! - **Data Persistence**: Maintains results in memory during program execution
//! - **Pretty Formatting**: Displays match results in a readable format
use std::fmt::Display;

enum MenuOption {
    Add,
    Search,
}

#[derive(Debug, Clone)]
struct Results {
    home_team: String,
    home_score: u32,
    away_team: String,
    away_score: u32,
}

impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} - {} {}",
            self.home_team, self.home_score, self.away_team, self.away_score
        )
    }
}

fn prompt_for_menu_opt() -> MenuOption {
    loop {
        println!("Enter 1 to add a result or 2 to search for a result: ");
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim() {
            "1" => return MenuOption::Add,
            "2" => return MenuOption::Search,
            _ => {
                println!("Invalid input. Please enter 1 or 2.");
                continue;
            }
        }
    }
}

fn prompt_for_result() -> Result<Results, Box<dyn std::error::Error>> {
    println!("Enter the home team: ");
    let mut home_team = String::new();
    std::io::stdin().read_line(&mut home_team)?;
    home_team = home_team.trim().to_string();

    println!("Enter the home team's score: ");
    let mut home_score = String::new();
    std::io::stdin().read_line(&mut home_score)?;
    let home_score: u32 = home_score.trim().parse()?;

    println!("Enter the away team: ");
    let mut away_team = String::new();
    std::io::stdin().read_line(&mut away_team)?;
    away_team = away_team.trim().to_string();

    println!("Enter the away team's score: ");
    let mut away_score = String::new();
    std::io::stdin().read_line(&mut away_score)?;
    let away_score: u32 = away_score.trim().parse()?;

    Ok(Results {
        home_team,
        home_score,
        away_team,
        away_score,
    })
}

fn prompt_for_query() -> String {
    println!("Enter the team name: ");
    let mut query = String::new();
    std::io::stdin().read_line(&mut query).unwrap();
    query.trim().to_string()
}

fn main() {
    const MAX_ITERATIONS: u32 = 20;
    let mut results: Vec<Results> = Vec::new();

    for _ in 0..MAX_ITERATIONS {
        let query_type = prompt_for_menu_opt();

        match query_type {
            MenuOption::Add => match prompt_for_result() {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Error: {}", e),
            },
            MenuOption::Search => {
                let query = prompt_for_query();

                println!("Search results for \"{}\":", query);
                let search_results: Vec<Results> = results
                    .iter()
                    .filter(|r| r.home_team == query || r.away_team == query)
                    .cloned()
                    .collect();
                if search_results.is_empty() {
                    println!("No results found.");
                } else {
                    search_results
                        .iter()
                        .for_each(|result| println!("{}", result));
                }
            }
        }
    }
}
