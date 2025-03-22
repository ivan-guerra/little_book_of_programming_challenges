//! # Voting Eligibility Calculator
//!
//! This module provides functionality to determine whether a person is eligible to vote
//! based on their birth date. It calculates the difference in years between the current
//! date and a provided birth date, and checks if the person meets the minimum voting age
//! requirement of 18 years.
//!
//! ## Features
//!
//! - Calculate the difference in years between dates
//! - Determine voting eligibility based on age
//! - Read and parse user input dates
//!
//! ## Usage
//!
//! Run the program and enter your birth date in YYYY-MM-DD format when prompted.
//! The program will inform you whether you are eligible to vote based on your age.
use chrono::{Local, NaiveDate};

fn get_years_difference(input_date: &NaiveDate) -> i64 {
    const DAYS_IN_YEAR: i64 = 365;
    let today = Local::now().date_naive();
    (today - *input_date).num_days() / DAYS_IN_YEAR
}

fn is_eligible_to_vote(birth_date: &NaiveDate) -> bool {
    const VOTING_AGE_LIMIT: i64 = 18;
    get_years_difference(birth_date) >= VOTING_AGE_LIMIT
}

fn read_user_date<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    let mut input = String::new();
    reader.read_line(&mut input)?;

    Ok(NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d")?)
}

fn main() {
    println!("Please enter your birth date (YYYY-MM-DD):");
    match read_user_date(&mut std::io::stdin().lock()) {
        Ok(birth_date) => {
            if is_eligible_to_vote(&birth_date) {
                println!("You are eligible to vote!");
            } else {
                println!("You are not eligible to vote.");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn get_years_difference_computes_past_date() {
        let past_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let years = get_years_difference(&past_date);
        assert!(
            years > 20,
            "Past date from 2000 should return more than 20 years, got: {}",
            years
        );
    }

    #[test]
    fn get_years_difference_computes_future_date() {
        let today = Local::now().date_naive();
        let future_date = today + chrono::Duration::days(366 * 2); // ~2 years in future
        let years = get_years_difference(&future_date);
        assert!(years < 0, "Future date should return negative years");
    }

    #[test]
    fn get_years_difference_handles_today() {
        let today = Local::now().date_naive();
        let years = get_years_difference(&today);
        assert_eq!(years, 0, "Today's date should return zero years");
    }

    #[test]
    fn get_years_difference_handles_less_than_year() {
        let today = Local::now().date_naive();
        let almost_year_ago = today - chrono::Duration::days(364);
        let years = get_years_difference(&almost_year_ago);
        assert_eq!(years, 0, "Less than a year ago should return zero years");
    }

    #[test]
    fn get_years_difference_handles_one_year() {
        let today = Local::now().date_naive();
        let one_year_ago = today - chrono::Duration::days(366);
        let years = get_years_difference(&one_year_ago);
        assert_eq!(years, 1, "One year ago should return one year");
    }

    #[test]
    fn get_years_difference_handles_leap_years() {
        let today = Local::now().date_naive();
        // About 4 years ago (including a leap year)
        let four_years_ago = today - chrono::Duration::days(365 * 4 + 1);
        let years = get_years_difference(&four_years_ago);
        assert_eq!(years, 4, "Four years ago should return four years");
    }

    #[test]
    fn is_eligible_to_vote_accepts_eligible_age() {
        let today = Local::now().date_naive();
        let birth_date = today - chrono::Duration::days(365 * 19); // 19 years old
        assert!(
            is_eligible_to_vote(&birth_date),
            "19-year-old should be eligible to vote"
        );
    }

    #[test]
    fn is_eligible_to_vote_rejects_underage() {
        let today = Local::now().date_naive();
        let birth_date = today - chrono::Duration::days(365 * 17); // 17 years old
        assert!(
            !is_eligible_to_vote(&birth_date),
            "17-year-old should not be eligible to vote"
        );
    }

    #[test]
    fn is_eligible_to_vote_accepts_exactly_18_years() {
        let today = Local::now().date_naive();
        let birth_date = today - chrono::Duration::days(365 * 18); // 18 years old
        assert!(
            is_eligible_to_vote(&birth_date),
            "18-year-old should be eligible to vote"
        );
    }
}
