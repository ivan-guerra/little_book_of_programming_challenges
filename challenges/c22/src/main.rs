//! # Random Array Generator and Visualizer
//!
//! This module implements a random 2D array generator and visualizer
//! that creates and displays numerical arrays with colorful representations.
//!
//! ## Features
//!
//! - **Random Generation**: Creates 2D arrays with random values in specified ranges
//! - **Numerical Display**: Outputs formatted numerical representation of arrays
//! - **Color Visualization**: Renders arrays using terminal background colors
//! - **Modular Design**: Separates generation and visualization concerns
//! - **Terminal Graphics**: Utilizes crossterm library for colorful terminal output
//! - **Customizable Dimensions**: Supports arbitrary square array sizes
use crossterm::{
    style::{Color, Print, SetBackgroundColor},
    ExecutableCommand,
};
use rand::Rng;

fn create_rand_2d_array(n: usize, num_rng: &(u32, u32)) -> Vec<Vec<u32>> {
    let mut arr = vec![vec![0; n]; n];
    arr.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|elem| {
            *elem = rand::rng().random_range(num_rng.0..=num_rng.1);
        });
    });
    arr
}

fn print_2d_array(arr: &[Vec<u32>]) {
    arr.iter().for_each(|row| {
        row.iter().for_each(|elem| {
            print!("{:4}", elem);
        });
        println!();
    });
}

fn print_2d_array_colored(arr: &[Vec<u32>]) {
    let mut stdout = std::io::stdout();
    const NUM_COLORS: u32 = 5;
    arr.iter().for_each(|row| {
        row.iter().for_each(|elem| {
            let color = match elem % NUM_COLORS {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                4 => Color::Magenta,
                _ => Color::White,
            };

            let _ = stdout.execute(SetBackgroundColor(color));
            let _ = stdout.execute(Print(' '));
        });
        println!();
    });
}

fn main() {
    let n = 10;
    let num_rng = (0, 15);
    let arr = create_rand_2d_array(n, &num_rng);
    print_2d_array(&arr);
    print_2d_array_colored(&arr);
}
