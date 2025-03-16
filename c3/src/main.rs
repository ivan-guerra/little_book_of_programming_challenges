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

fn read_input<R: std::io::BufRead>(reader: &mut R, prompt: &str) -> Result<String, std::io::Error> {
    use std::io::Write;
    if !prompt.is_empty() {
        print!("{}", prompt);
        std::io::stdout().flush()?;
    }
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input)
}

fn prompt_for_shape<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Shape, Box<dyn std::error::Error>> {
    println!("Enter 1 for Rectangle, 2 for Cuboid");
    let choice: u32 = read_input(reader, "")?.trim().parse()?;

    match choice {
        1 => {
            let width = read_input(reader, "Enter width: ")?.trim().parse()?;
            let height = read_input(reader, "Enter height: ")?.trim().parse()?;
            Ok(Shape::Rectangle { width, height })
        }
        2 => {
            let width = read_input(reader, "Enter width: ")?.trim().parse()?;
            let height = read_input(reader, "Enter height: ")?.trim().parse()?;
            let depth = read_input(reader, "Enter depth: ")?.trim().parse()?;
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

    // Helper function to create a mock input reader
    fn create_test_input(input: &str) -> BufReader<std::io::Cursor<Vec<u8>>> {
        let cursor = std::io::Cursor::new(input.as_bytes().to_vec());
        BufReader::new(cursor)
    }

    #[test]
    fn when_calculating_rectangle_area_then_returns_correct_result() {
        assert_eq!(rect_area(2.0, 3.0), 6.0);
        assert_eq!(rect_area(0.0, 5.0), 0.0);
        assert_eq!(rect_area(2.5, 4.0), 10.0);
    }

    #[test]
    fn when_calculating_cuboid_volume_then_returns_correct_result() {
        assert_eq!(cuboid_volume(2.0, 3.0, 4.0), 24.0);
        assert_eq!(cuboid_volume(0.0, 5.0, 2.0), 0.0);
        assert_eq!(cuboid_volume(2.5, 4.0, 2.0), 20.0);
    }

    #[test]
    fn when_reading_input_then_returns_trimmed_string() -> Result<(), Box<dyn std::error::Error>> {
        let input = "test\n";
        let mut reader = create_test_input(input);
        let result = read_input(&mut reader, "")?;
        assert_eq!(result.trim(), "test");
        Ok(())
    }

    #[test]
    fn when_prompting_for_rectangle_then_creates_correct_shape(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input = "1\n2.5\n3.0\n";
        let mut reader = create_test_input(input);
        let expected = Shape::Rectangle {
            width: 2.5,
            height: 3.0,
        };
        let shape = prompt_for_shape(&mut reader)?;
        assert_eq!(shape, expected);
        Ok(())
    }

    #[test]
    fn when_prompting_for_cuboid_then_creates_correct_shape(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input = "2\n2.5\n3.0\n4.0\n";
        let mut reader = create_test_input(input);
        let expected = Shape::Cuboid {
            width: 2.5,
            height: 3.0,
            depth: 4.0,
        };
        let shape = prompt_for_shape(&mut reader)?;
        assert_eq!(shape, expected);
        Ok(())
    }

    #[test]
    fn when_shape_choice_invalid_then_returns_error() {
        let input = "3\n";
        let mut reader = create_test_input(input);
        let result = prompt_for_shape(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn when_input_not_numeric_then_returns_error() {
        let input = "abc\n";
        let mut reader = create_test_input(input);
        let result = prompt_for_shape(&mut reader);
        assert!(result.is_err());
    }
}
