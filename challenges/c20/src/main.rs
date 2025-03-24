//! # Fibonacci Calculator
//!
//! This module implements a simple interactive Fibonacci number calculator
//! that computes Fibonacci sequence values at specified indices.
//!
//! ## Features
//!
//! - **Efficient Computation**: Calculates Fibonacci numbers using an iterative approach
//! - **Large Number Support**: Handles large Fibonacci numbers up to the 50th value using u128
//! - **Memory Optimization**: Uses constant space regardless of input size
type FibIndex = u8;

fn fib(n: FibIndex) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    (2..=n).for_each(|_| {
        c = a + b;
        a = b;
        b = c;
    });
    c
}

fn prompt_for_index() -> FibIndex {
    loop {
        let mut input = String::new();
        println!("Enter the index of the Fibonacci number: ");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().parse() {
            Ok(num) => return num,
            Err(e) => eprintln!("Error: {}. Please enter a valid number.", e),
        }
    }
}

fn main() {
    let index = prompt_for_index();
    println!("Fibonacci number at index {}: {}", index, fib(index));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_returns_zero_for_index_zero() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn fib_returns_one_for_index_one() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn fib_calculates_small_fibonacci_numbers_correctly() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
    }

    #[test]
    fn fib_calculates_medium_fibonacci_numbers_correctly() {
        assert_eq!(fib(10), 55);
        assert_eq!(fib(15), 610);
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn fib_calculates_large_fibonacci_numbers_correctly() {
        assert_eq!(fib(30), 832040);
        assert_eq!(fib(40), 102334155);
        assert_eq!(fib(50), 12586269025);
    }
}
