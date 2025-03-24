//! # Treasure Hunt Game
//!
//! This module implements an interactive treasure hunt game
//! where players search for hidden treasures on a 2D grid.
//!
//! ## Features
//!
//! - **Random Generation**: Creates random treasure locations on a grid
//! - **Proximity Hints**: Provides "hot/warm/cold" feedback based on distance
//! - **Distance Calculation**: Uses Euclidean distance to measure proximity
//! - **Input Validation**: Ensures coordinates are within the grid boundaries
//! - **Error Handling**: Provides clear feedback for invalid inputs
//! - **Interactive Gameplay**: Continues until the treasure is found
use rand::Rng;

type Point2D = (u32, u32);

enum Proximity {
    Hot,
    Warm,
    Cold,
}

fn generate_random_coord(size: u32) -> (u32, u32) {
    let mut rng = rand::rng();
    (rng.random_range(0..size), rng.random_range(0..size))
}

fn calculate_2d_distance(p1: Point2D, p2: Point2D) -> f64 {
    let x_diff = f64::from(p1.0) - f64::from(p2.0);
    let y_diff = f64::from(p1.1) - f64::from(p2.1);
    (x_diff.powi(2) + y_diff.powi(2)).sqrt()
}

fn prompt_for_location(size: u32) -> Point2D {
    println!("Enter the x,y location of the treasure: ");
    loop {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let coords: Vec<&str> = input.trim().split(',').collect();
        if coords.len() != 2 {
            println!("Invalid input. Please enter two numbers separated by a comma.");
            continue;
        }

        match (coords[0].parse(), coords[1].parse()) {
            (Ok(x), Ok(y)) => {
                if x >= size || y >= size {
                    println!(
                        "Coordinates out of bounds. Please enter values within the grid size."
                    );
                    continue;
                }
                return (x, y);
            }
            _ => {
                println!("Invalid input. Please enter two numbers separated by a comma.");
                continue;
            }
        };
    }
}

fn get_proximity(size: u32, p1: Point2D, p2: Point2D) -> Proximity {
    let distance = calculate_2d_distance(p1, p2);
    let hot_radius = f64::from(size) * 0.25;
    let warm_radius = f64::from(size) * 0.5;
    if distance <= hot_radius {
        Proximity::Hot
    } else if distance <= warm_radius {
        Proximity::Warm
    } else {
        Proximity::Cold
    }
}

fn main() {
    const MAP_SIZE: u32 = 10;
    println!(
        "This is a game where you guess the x,y location of treasure on a {}x{} grid.",
        MAP_SIZE, MAP_SIZE
    );
    println!("Make your guesses and follow the hints to find the treasure!");

    let treasure = generate_random_coord(MAP_SIZE);
    loop {
        let guess = prompt_for_location(MAP_SIZE);
        if guess == treasure {
            println!("Congratulations! You found the treasure!");
            break;
        }

        match get_proximity(MAP_SIZE, guess, treasure) {
            Proximity::Hot => println!("You're hot!"),
            Proximity::Warm => println!("You're warm!"),
            Proximity::Cold => println!("You're cold!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_2d_distance_returns_zero_for_same_points() {
        assert_eq!(calculate_2d_distance((5, 5), (5, 5)), 0.0);
    }

    #[test]
    fn calculate_2d_distance_calculates_horizontal_distance_correctly() {
        assert_eq!(calculate_2d_distance((0, 0), (3, 0)), 3.0);
        assert_eq!(calculate_2d_distance((5, 7), (10, 7)), 5.0);
    }

    #[test]
    fn calculate_2d_distance_calculates_vertical_distance_correctly() {
        assert_eq!(calculate_2d_distance((0, 0), (0, 4)), 4.0);
        assert_eq!(calculate_2d_distance((8, 2), (8, 7)), 5.0);
    }

    #[test]
    fn calculate_2d_distance_calculates_diagonal_distance_correctly() {
        assert_eq!(calculate_2d_distance((0, 0), (3, 4)), 5.0);
        assert_eq!(calculate_2d_distance((1, 1), (4, 5)), 5.0);
    }

    #[test]
    fn calculate_2d_distance_handles_large_coordinates() {
        let result = calculate_2d_distance((100, 100), (104, 103));
        assert!((result - 5.0).abs() < 0.00001);
    }

    #[test]
    fn calculate_2d_distance_is_commutative() {
        let point1 = (3, 7);
        let point2 = (8, 2);
        let distance1 = calculate_2d_distance(point1, point2);
        let distance2 = calculate_2d_distance(point2, point1);
        assert_eq!(distance1, distance2);
    }

    #[test]
    fn get_proximity_returns_hot_for_close_points() {
        // Within 25% of the size
        let size = 10;
        let hot_threshold = (size as f64 * 0.25) as u32;

        // Test at exact threshold
        assert!(matches!(
            get_proximity(size, (5, 5), (5, 5 + hot_threshold)),
            Proximity::Hot
        ));

        // Test well within threshold
        assert!(matches!(
            get_proximity(size, (5, 5), (6, 6)),
            Proximity::Hot
        ));
    }

    #[test]
    fn get_proximity_returns_warm_for_medium_distance_points() {
        // Between 25% and 50% of the size
        let size = 10;
        let hot_threshold = (size as f64 * 0.25) as u32;
        let warm_threshold = (size as f64 * 0.5) as u32;

        // Test just outside hot threshold
        assert!(matches!(
            get_proximity(size, (5, 5), (5, 5 + hot_threshold + 1)),
            Proximity::Warm
        ));

        // Test at warm threshold
        assert!(matches!(
            get_proximity(size, (5, 5), (5, 5 + warm_threshold)),
            Proximity::Warm
        ));
    }

    #[test]
    fn get_proximity_returns_cold_for_distant_points() {
        // Beyond 50% of the size
        let size = 10;
        let warm_threshold = (size as f64 * 0.5) as u32;

        // Test just outside warm threshold
        assert!(matches!(
            get_proximity(size, (5, 5), (5, 5 + warm_threshold + 1)),
            Proximity::Cold
        ));

        // Test at maximum distance
        assert!(matches!(
            get_proximity(size, (0, 0), (size - 1, size - 1)),
            Proximity::Cold
        ));
    }
}
