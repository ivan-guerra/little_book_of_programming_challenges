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

fn prompt_for_query() -> Result<Query, Box<dyn std::error::Error>> {
    print!("Enter query type (1:distance, 2:speed): ");
    std::io::stdout().flush()?;

    let validate_time = |time: f64| -> Result<(), String> {
        if time <= 0.0 {
            Err("Time must be positive".to_string())
        } else {
            Ok(())
        }
    };
    let validate_speed = |time: f64| -> Result<(), String> {
        if time < 0.0 {
            Err("Speed cannot be negative".to_string())
        } else {
            Ok(())
        }
    };
    let validate_distance = |distance: f64| -> Result<(), String> {
        if distance < 0.0 {
            Err("Distance cannot be negative".to_string())
        } else {
            Ok(())
        }
    };

    let query_type = read_input()?;
    match query_type.as_str() {
        "1" => {
            let speed_mph = prompt_number("Enter speed (mph): ")?;
            validate_speed(speed_mph)?;
            let time_hr = prompt_number("Enter time (hours): ")?;
            validate_time(time_hr)?;
            Ok(Query::Distance { speed_mph, time_hr })
        }
        "2" => {
            let distance_miles = prompt_number("Enter distance (miles): ")?;
            validate_distance(distance_miles)?;
            let time_hr = prompt_number("Enter time (hours): ")?;
            validate_time(time_hr)?;
            Ok(Query::Speed {
                distance_miles,
                time_hr,
            })
        }
        _ => Err("Invalid input. Please enter 1 or 2.".into()),
    }
}

fn read_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_number(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let input = read_input()?;
    Ok(input.parse()?)
}

fn main() {
    let query = prompt_for_query().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let result = calculate_query(&query);
    println!(
        "{}: {:.2} {}",
        get_result_label(&query),
        result.value,
        result.unit
    );
}

fn get_result_label(query: &Query) -> &'static str {
    match query {
        Query::Distance { .. } => "Distance",
        Query::Speed { .. } => "Speed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_speed_is_60mph_for_2hours_distance_is_120miles() {
        let query = Query::Distance {
            speed_mph: 60.0,
            time_hr: 2.0,
        };
        let result = calculate_query(&query);
        assert_eq!(
            result,
            CalculationResult {
                value: 120.0,
                unit: "miles".to_string()
            }
        );
    }

    #[test]
    fn when_traveling_120miles_in_2hours_speed_is_60mph() {
        let query = Query::Speed {
            distance_miles: 120.0,
            time_hr: 2.0,
        };
        let result = calculate_query(&query);
        assert_eq!(
            result,
            CalculationResult {
                value: 60.0,
                unit: "mph".to_string()
            }
        );
    }

    #[test]
    fn when_querying_labels_distance_and_speed_queries_return_correct_text() {
        let distance_query = Query::Distance {
            speed_mph: 60.0,
            time_hr: 2.0,
        };
        let speed_query = Query::Speed {
            distance_miles: 120.0,
            time_hr: 2.0,
        };

        assert_eq!(get_result_label(&distance_query), "Distance");
        assert_eq!(get_result_label(&speed_query), "Speed");
    }
}
