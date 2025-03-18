//! A date calculation utility that computes the difference between dates.
//!
//! This module provides functionality to calculate the time difference between dates
//! in both days and seconds. It includes interactive input handling for date entry
//! in the YYYY-MM-DD format and proper error handling for invalid inputs.
use chrono::{Local, NaiveDate};

fn get_days_difference(input_date: &NaiveDate) -> i64 {
    let today = Local::now().date_naive();
    (today - *input_date).num_days()
}

fn get_seconds_difference(input_date: &NaiveDate) -> i64 {
    let today = Local::now().date_naive();
    (today - *input_date).num_seconds()
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
        Ok(date) => {
            println!("Days difference: {}", get_days_difference(&date));
            println!("Seconds difference: {}", get_seconds_difference(&date));
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
    use std::io::BufReader;

    #[test]
    fn get_days_difference_computes_past_date() {
        let past_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let days = get_days_difference(&past_date);
        assert!(
            days > 0,
            "Past date should return positive days, got: {}",
            days
        );
    }

    #[test]
    fn get_days_difference_computes_future_date() {
        let today = Local::now().date_naive();
        let future_date = today + chrono::Duration::days(7);
        let days = get_days_difference(&future_date);
        assert_eq!(days, -7, "Future date should return negative days");
    }

    #[test]
    fn get_days_difference_handles_today() {
        let today = Local::now().date_naive();
        let days = get_days_difference(&today);
        assert_eq!(days, 0, "Today's date should return zero days");
    }

    #[test]
    fn get_days_difference_handles_yesterday() {
        let today = Local::now().date_naive();
        let yesterday = today - chrono::Duration::days(1);
        let days = get_days_difference(&yesterday);
        assert_eq!(days, 1, "Yesterday should return one day");
    }

    #[test]
    fn get_seconds_difference_computes_past_date() {
        let past_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let seconds = get_seconds_difference(&past_date);
        assert!(
            seconds > 0,
            "Past date should return positive seconds, got: {}",
            seconds
        );
    }

    #[test]
    fn get_seconds_difference_computes_future_date() {
        let today = Local::now().date_naive();
        let future_date = today + chrono::Duration::days(1);
        let seconds = get_seconds_difference(&future_date);
        assert!(seconds < 0, "Future date should return negative seconds");
        assert_eq!(
            seconds, -86400,
            "One day in the future should be -86400 seconds"
        );
    }

    #[test]
    fn get_seconds_difference_handles_today() {
        let today = Local::now().date_naive();
        let seconds = get_seconds_difference(&today);
        assert_eq!(seconds, 0, "Today's date should return zero seconds");
    }

    #[test]
    fn get_seconds_difference_handles_yesterday() {
        let today = Local::now().date_naive();
        let yesterday = today - chrono::Duration::days(1);
        let seconds = get_seconds_difference(&yesterday);
        assert_eq!(seconds, 86400, "One day ago should be 86400 seconds");
    }

    #[test]
    fn read_user_date_accepts_valid_date() {
        let input = "2023-12-25\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = read_user_date(&mut reader);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
    }

    #[test]
    fn read_user_date_rejects_invalid_format() {
        let input = "12/25/2023\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = read_user_date(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn read_user_date_rejects_invalid_date() {
        let input = "2023-13-45\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = read_user_date(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn read_user_date_rejects_empty_input() {
        let input = "\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = read_user_date(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn read_user_date_rejects_non_date_input() {
        let input = "not a date\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = read_user_date(&mut reader);

        assert!(result.is_err());
    }
}
