//! A speed and distance calculator that provides interactive calculations for:
//!
//! - Distance (given speed and time)
//! - Speed (given distance and time)
//!
//! # Usage
//!
//! The program prompts the user to:
//! 1. Select calculation type (distance or speed)
//! 2. Input required parameters (speed/distance and time)
use std::io::Write;

#[derive(Debug, PartialEq)]
enum Query {
    Distance { speed_mph: f64, time_hr: f64 },
    Speed { distance_miles: f64, time_hr: f64 },
}

#[derive(Debug, PartialEq)]
struct CalculationResult {
    value: f64,
    unit: String,
}

fn calculate_query(query: &Query) -> CalculationResult {
    match query {
        Query::Distance { speed_mph, time_hr } => CalculationResult {
            value: speed_mph * time_hr,
            unit: "miles".to_string(),
        },
        Query::Speed {
            distance_miles,
            time_hr,
        } => CalculationResult {
            value: distance_miles / time_hr,
            unit: "mph".to_string(),
        },
    }
}

fn read_input<R: std::io::BufRead>(reader: &mut R) -> Result<String, std::io::Error> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_for_param<R: std::io::BufRead>(
    reader: &mut R,
    param_name: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    print!("Enter {}: ", param_name);
    std::io::stdout().flush()?;
    let input = read_input(reader)?;

    let value = input.parse()?;
    if value <= 0.0 {
        return Err(format!(" {param_name} must be positive").into());
    }

    Ok(value)
}

fn prompt_for_query<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Query, Box<dyn std::error::Error>> {
    print!("Enter query type (1:distance, 2:speed): ");
    std::io::stdout().flush()?;
    let query_type = read_input(reader)?;

    match query_type.as_str() {
        "1" => {
            let speed_mph = prompt_for_param(reader, "speed (mph)")?;
            let time_hr = prompt_for_param(reader, "time (hours)")?;
            Ok(Query::Distance { speed_mph, time_hr })
        }
        "2" => {
            let distance_miles = prompt_for_param(reader, "distance (miles)")?;
            let time_hr = prompt_for_param(reader, "time (hours)")?;
            Ok(Query::Speed {
                distance_miles,
                time_hr,
            })
        }
        _ => Err("Invalid input. Please enter 1 or 2.".into()),
    }
}

fn main() {
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    let query = prompt_for_query(&mut stdin).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let result = calculate_query(&query);
    let metric_type = match query {
        Query::Distance { .. } => "Distance",
        Query::Speed { .. } => "Speed",
    };
    println!("{}: {:.2} {}", metric_type, result.value, result.unit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn calculate_query_computes_distance() {
        let query = Query::Distance {
            speed_mph: 60.0,
            time_hr: 2.0,
        };

        let result = calculate_query(&query);

        assert_eq!(result.value, 120.0);
        assert_eq!(result.unit, "miles");
    }

    #[test]
    fn calculate_query_computes_speed() {
        let query = Query::Speed {
            distance_miles: 120.0,
            time_hr: 2.0,
        };

        let result = calculate_query(&query);

        assert_eq!(result.value, 60.0);
        assert_eq!(result.unit, "mph");
    }

    #[test]
    fn calculate_query_handles_zero_time() {
        let query = Query::Speed {
            distance_miles: 100.0,
            time_hr: 0.0,
        };

        let result = calculate_query(&query);

        assert!(result.value.is_infinite());
        assert_eq!(result.unit, "mph");
    }

    #[test]
    fn calculate_query_handles_zero_distance() {
        let query = Query::Speed {
            distance_miles: 0.0,
            time_hr: 2.0,
        };

        let result = calculate_query(&query);

        assert_eq!(result.value, 0.0);
        assert_eq!(result.unit, "mph");
    }

    #[test]
    fn calculate_query_handles_fractional_values() {
        let query = Query::Distance {
            speed_mph: 0.5,
            time_hr: 0.5,
        };

        let result = calculate_query(&query);

        assert_eq!(result.value, 0.25);
        assert_eq!(result.unit, "miles");
    }

    #[test]
    fn prompt_for_param_accepts_valid_positive_number() {
        let input = "42.5\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_param(&mut reader, "test_param");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.5);
    }

    #[test]
    fn prompt_for_param_rejects_negative_number() {
        let input = "-5.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_param(&mut reader, "test_param");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            " test_param must be positive"
        );
    }

    #[test]
    fn prompt_for_param_rejects_zero() {
        let input = "0.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_param(&mut reader, "test_param");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            " test_param must be positive"
        );
    }

    #[test]
    fn prompt_for_param_rejects_non_numeric_input() {
        let input = "not_a_number\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_param(&mut reader, "test_param");
        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_query_handles_distance_query() {
        let input = "1\n10.0\n2.5\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_query(&mut reader).unwrap();
        match result {
            Query::Distance { speed_mph, time_hr } => {
                assert_eq!(speed_mph, 10.0);
                assert_eq!(time_hr, 2.5);
            }
            _ => panic!("Expected Distance query"),
        }
    }

    #[test]
    fn prompt_for_query_handles_speed_query() {
        let input = "2\n100.0\n2.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_query(&mut reader).unwrap();
        match result {
            Query::Speed {
                distance_miles,
                time_hr,
            } => {
                assert_eq!(distance_miles, 100.0);
                assert_eq!(time_hr, 2.0);
            }
            _ => panic!("Expected Speed query"),
        }
    }

    #[test]
    fn prompt_for_query_rejects_invalid_query_type() {
        let input = "3\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_query(&mut reader);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid input. Please enter 1 or 2."
        );
    }

    #[test]
    fn prompt_for_query_rejects_invalid_numeric_input() {
        let input = "1\nabc\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_query(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_query_rejects_negative_values() {
        let input = "1\n-10.0\n2.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_query(&mut reader);
        assert!(result.is_err());
    }
}
