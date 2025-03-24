//! # UMS Score Converter
//!
//! This module implements a simple interactive UMS (Uniform Mark Scale) score converter
//! that translates numerical scores into letter grades for educational assessment.
//!
//! ## Features
//!
//! - **Score Conversion**: Converts UMS scores between 0-100 to letter grades A-F
//! - **Multi-Module Support**: Handles scores for two separate modules
//! - **Average Calculation**: Computes an overall AS Level grade based on module averages
//! - **Input Validation**: Ensures all scores are within the valid UMS range (0-100)
//! - **Error Handling**: Provides clear feedback for invalid inputs
type UmsScore = u32;
const MAX_SCORE: UmsScore = 100;

fn ums_to_grade(ums: UmsScore) -> Result<char, Box<dyn std::error::Error>> {
    let grade = match ums {
        80..=100 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        50..=59 => 'D',
        0..=49 => 'F',
        _ => return Err("UMS score out of range.".into()),
    };

    Ok(grade)
}

fn prompt_for_module_result(prompt: &str) -> UmsScore {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().parse() {
            Ok(num) => {
                if num > MAX_SCORE {
                    println!(
                        "Invalid input. Please enter a number between 0 and {}.",
                        MAX_SCORE
                    );
                    continue;
                }
                return num;
            }
            Err(e) => {
                eprintln!(
                    "Error: {}. Please enter a number between 0 and {}.",
                    e, MAX_SCORE
                );
            }
        }
    }
}

fn print_results(module1: UmsScore, module2: UmsScore) -> Result<(), Box<dyn std::error::Error>> {
    println!("Result: ");
    println!("Module 1: {}", ums_to_grade(module1)?);
    println!("Module 2: {}", ums_to_grade(module2)?);

    let overall_grade = ums_to_grade((module1 + module2) / 2)?;
    println!("AS Level: {}", overall_grade);

    Ok(())
}

fn main() {
    let module1 = prompt_for_module_result("Enter UMS score for Module 1: ");
    let module2 = prompt_for_module_result("Enter UMS score for Module 2: ");
    if let Err(e) = print_results(module1, module2) {
        eprintln!("Error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ums_to_grade_returns_a_for_scores_between_80_and_100() {
        assert_eq!(ums_to_grade(80).unwrap(), 'A');
        assert_eq!(ums_to_grade(90).unwrap(), 'A');
        assert_eq!(ums_to_grade(100).unwrap(), 'A');
    }

    #[test]
    fn ums_to_grade_returns_b_for_scores_between_70_and_79() {
        assert_eq!(ums_to_grade(70).unwrap(), 'B');
        assert_eq!(ums_to_grade(75).unwrap(), 'B');
        assert_eq!(ums_to_grade(79).unwrap(), 'B');
    }

    #[test]
    fn ums_to_grade_returns_c_for_scores_between_60_and_69() {
        assert_eq!(ums_to_grade(60).unwrap(), 'C');
        assert_eq!(ums_to_grade(65).unwrap(), 'C');
        assert_eq!(ums_to_grade(69).unwrap(), 'C');
    }

    #[test]
    fn ums_to_grade_returns_d_for_scores_between_50_and_59() {
        assert_eq!(ums_to_grade(50).unwrap(), 'D');
        assert_eq!(ums_to_grade(55).unwrap(), 'D');
        assert_eq!(ums_to_grade(59).unwrap(), 'D');
    }

    #[test]
    fn ums_to_grade_returns_f_for_scores_between_0_and_49() {
        assert_eq!(ums_to_grade(0).unwrap(), 'F');
        assert_eq!(ums_to_grade(25).unwrap(), 'F');
        assert_eq!(ums_to_grade(49).unwrap(), 'F');
    }

    #[test]
    fn ums_to_grade_returns_error_for_scores_above_100() {
        assert!(ums_to_grade(101).is_err());
        assert!(ums_to_grade(150).is_err());
    }
}
