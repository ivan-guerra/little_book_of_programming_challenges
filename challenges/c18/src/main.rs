//! # Pyramid Generator
//!
//! This module implements a simple interactive pyramid generator
//! that creates ASCII pyramids based on user input.
//!
//! ## Features
//!
//! - **ASCII Art**: Generates pyramids of stars with proper spacing
//! - **Input Validation**: Ensures the base is an odd number
//! - **Error Handling**: Provides clear feedback for invalid inputs
//! - **String Formatting**: Handles proper alignment of pyramid elements

fn draw_stars(num_spaces: u32, num_stars: u32) -> String {
    let spaces = " ".repeat(num_spaces as usize);
    let stars = "*".repeat(num_stars as usize);
    let line = format!("{}{}", spaces, stars);
    line
}

fn draw_pyramid(base: u32) {
    (0..base).for_each(|i| {
        let num_spaces = base - i - 1;
        let num_stars = 2 * i + 1;
        println!("{}", draw_stars(num_spaces, num_stars));
    })
}

fn prompt_for_base() -> u32 {
    loop {
        let mut input = String::new();
        println!("Enter the base of the pyramid: ");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().parse() {
            Ok(num) => {
                if num % 2 == 0 {
                    println!("Invalid input. Please enter an odd number.");
                    continue;
                }
                return num;
            }
            Err(e) => {
                eprintln!("Error: {}. Please enter a valid number.", e);
            }
        }
    }
}

fn main() {
    let base = prompt_for_base();
    draw_pyramid(base);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_stars_returns_correct_string_with_zero_spaces() {
        assert_eq!(draw_stars(0, 5), "*****");
    }

    #[test]
    fn draw_stars_returns_correct_string_with_zero_stars() {
        assert_eq!(draw_stars(3, 0), "   ");
    }

    #[test]
    fn draw_stars_returns_correct_string_with_spaces_and_stars() {
        assert_eq!(draw_stars(3, 5), "   *****");
    }

    #[test]
    fn draw_stars_returns_empty_string_with_zero_spaces_and_stars() {
        assert_eq!(draw_stars(0, 0), "");
    }

    #[test]
    fn draw_stars_handles_large_numbers_correctly() {
        assert_eq!(draw_stars(10, 10), "          **********");
    }
}
