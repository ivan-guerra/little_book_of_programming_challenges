//! # Factor Calculator
//!
//! This module implements a utility for finding all factors of a given number.
//! It provides functionality to calculate and display the complete set of factors
//! for any non-negative integer input.
//!
//! ## Features
//!
//! - Efficient factor calculation using square root optimization
//! - Interactive command-line interface for user input
//! - Support for large numbers
//! - Handles special cases (zero, one, prime numbers)
//! - Clear display of all factors

fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                // Avoid duplicate for perfect squares
                result.push(n / i);
            }
        }
    }

    result.sort();
    result
}

fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("Error: {}", e);
        return;
    }
    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if primal::is_prime(input) {
        println!("{input} is a prime number, its factors are 1 and {input}.");
    } else {
        let factors = factors(input);
        println!("Factors of {} are: {:?}", input, factors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_returns_empty_vec_for_zero() {
        assert_eq!(factors(0), Vec::<u64>::new());
    }

    #[test]
    fn factors_returns_one_for_one() {
        assert_eq!(factors(1), vec![1]);
    }

    #[test]
    fn factors_returns_prime_and_self_for_primes() {
        assert_eq!(factors(2), vec![1, 2]);
        assert_eq!(factors(3), vec![1, 3]);
        assert_eq!(factors(5), vec![1, 5]);
        assert_eq!(factors(7), vec![1, 7]);
        assert_eq!(factors(11), vec![1, 11]);
    }

    #[test]
    fn factors_returns_all_factors_for_composite_numbers() {
        assert_eq!(factors(4), vec![1, 2, 4]);
        assert_eq!(factors(6), vec![1, 2, 3, 6]);
        assert_eq!(factors(8), vec![1, 2, 4, 8]);
        assert_eq!(factors(9), vec![1, 3, 9]);
        assert_eq!(factors(12), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn factors_returns_correct_for_perfect_squares() {
        assert_eq!(factors(16), vec![1, 2, 4, 8, 16]);
        assert_eq!(factors(25), vec![1, 5, 25]);
        assert_eq!(factors(36), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
    }

    #[test]
    fn factors_handles_large_numbers() {
        assert_eq!(factors(100), vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
        assert_eq!(factors(997), vec![1, 997]); // 997 is prime
        assert_eq!(factors(1001), vec![1, 7, 11, 13, 77, 91, 143, 1001]);
    }
}
