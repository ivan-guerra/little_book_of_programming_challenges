//! A geometric shape calculator that computes areas and volumes.
//!
//! This program allows users to calculate:
//! - Rectangle areas by providing width and height
//! - Cuboid volumes by providing width, height, and depth
//!
//! # Features
//! - Interactive command-line interface
//! - Input validation for dimensions (positive numbers only)
//! - Error handling for invalid inputs
//! - Support for floating-point dimensions
//!
//! # Usage
//! The program prompts users to:
//! 1. Choose a shape type (1 for Rectangle, 2 for Cuboid)
//! 2. Enter dimensions when prompted
//! 3. Displays the calculated area or volume
//!
//! # Error Handling
//! The program validates all inputs and handles:
//! - Non-numeric inputs
//! - Negative dimensions
//! - Zero dimensions
//! - Invalid shape choices
use std::io::Write;

#[derive(Debug, PartialEq)]
enum Shape {
    Rectangle { width: f64, height: f64 },
    Cuboid { width: f64, height: f64, depth: f64 },
}

fn rect_area(width: f64, height: f64) -> f64 {
    width * height
}

fn cuboid_volume(width: f64, height: f64, depth: f64) -> f64 {
    width * height * depth
}

fn prompt_for_dimension<R: std::io::BufRead>(
    reader: &mut R,
    dimension: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    print!("Enter {}:", dimension);
    std::io::stdout().flush()?;

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let dim = input.trim().parse()?;
    if dim <= 0.0 {
        return Err(format!("{} must be greater than zero", dimension).into());
    }

    Ok(dim)
}

fn prompt_for_shape<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Shape, Box<dyn std::error::Error>> {
    println!("Enter 1 for Rectangle, 2 for Cuboid");
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let choice: u32 = input.trim().parse()?;

    let width = prompt_for_dimension(reader, "width")?;
    let height = prompt_for_dimension(reader, "height")?;
    match choice {
        1 => Ok(Shape::Rectangle { width, height }),
        2 => {
            let depth = prompt_for_dimension(reader, "depth")?;
            Ok(Shape::Cuboid {
                width,
                height,
                depth,
            })
        }
        _ => Err("Invalid choice".into()),
    }
}

fn main() {
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    let shape = prompt_for_shape(&mut stdin);
    match shape {
        Ok(shape) => match shape {
            Shape::Rectangle { width, height } => {
                let area = rect_area(width, height);
                println!("Area: {}", area);
            }
            Shape::Cuboid {
                width,
                height,
                depth,
            } => {
                let volume = cuboid_volume(width, height, depth);
                println!("Volume: {}", volume);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn prompt_for_dimension_accepts_positive_input() {
        let input = "5.5\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_dimension(&mut reader, "width");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.5);
    }

    #[test]
    fn prompt_for_dimension_rejects_zero() {
        let input = "0.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_dimension(&mut reader, "height");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "height must be greater than zero"
        );
    }

    #[test]
    fn prompt_for_dimension_rejects_negative_values() {
        let input = "-2.5\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_dimension(&mut reader, "length");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "length must be greater than zero"
        );
    }

    #[test]
    fn prompt_for_dimension_rejects_non_numeric_input() {
        let input = "not_a_number\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_dimension(&mut reader, "width");
        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_dimension_rejects_empty_input() {
        let input = "\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_dimension(&mut reader, "height");
        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_shape_creates_valid_rectangle() {
        let input = "1\n5.0\n3.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_ok());
        if let Ok(Shape::Rectangle { width, height }) = result {
            assert_eq!(width, 5.0);
            assert_eq!(height, 3.0);
        } else {
            panic!("Expected Rectangle shape");
        }
    }

    #[test]
    fn prompt_for_shape_creates_valid_cuboid() {
        let input = "2\n2.0\n3.0\n4.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_ok());
        if let Ok(Shape::Cuboid {
            width,
            height,
            depth,
        }) = result
        {
            assert_eq!(width, 2.0);
            assert_eq!(height, 3.0);
            assert_eq!(depth, 4.0);
        } else {
            panic!("Expected Cuboid shape");
        }
    }

    #[test]
    fn prompt_for_shape_rejects_invalid_choice() {
        let input = "3\n2.0\n3.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid choice");
    }

    #[test]
    fn prompt_for_shape_rejects_non_numeric_input() {
        let input = "abc\n2.0\n3.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_shape_rejects_negative_dimensions() {
        let input = "1\n-2.0\n3.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_shape_rejects_zero_dimensions() {
        let input = "2\n2.0\n0.0\n4.0\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn prompt_for_shape_rejects_empty_input() {
        let input = "\n";
        let mut reader = BufReader::new(input.as_bytes());
        let result = prompt_for_shape(&mut reader);

        assert!(result.is_err());
    }
}
