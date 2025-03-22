//! # Random Card Generator
//!
//! This module implements a simple program that generates random playing cards.
//! It allows users to repeatedly generate cards from a standard deck and
//! decide whether to continue or exit.
//!
//! ## Features
//!
//! - Generates random playing cards with suits (Hearts, Diamonds, Clubs, Spades)
//! - Generates random card ranks (Ace through King)
//! - Provides deterministic functions that accept random number generators for testing
//! - Includes comprehensive test suite to verify randomness and distribution
//!
//! The implementation ensures even distribution of both ranks and suits over
//! a large number of generations, as verified by the test suite.
use rand::seq::IndexedRandom;
use rand::Rng;
fn get_rand_suite_with_rng<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    static SUITES: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];
    SUITES.choose(rng).unwrap_or(&"Hearts")
}

fn get_rand_suite() -> &'static str {
    get_rand_suite_with_rng(&mut rand::rng())
}

fn get_rand_rank_with_rng<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    static RANKS: [&str; 13] = [
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
    ];
    RANKS.choose(rng).unwrap_or(&"Ace")
}

fn get_rand_rank() -> &'static str {
    get_rand_rank_with_rng(&mut rand::rng())
}

fn main() {
    println!("This program generates a random card from a deck of cards.");
    loop {
        println!("Your card is: {} of {}", get_rand_rank(), get_rand_suite());

        println!("Do you want another card? (yes/no)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "yes" {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn get_rand_suite_returns_valid_suite_with_seeded_rng() {
        let mut seeded_rng = StdRng::seed_from_u64(42); // Deterministic seed
        let mut results = HashSet::new();

        // Run multiple times to collect different results
        for _ in 0..20 {
            results.insert(get_rand_suite_with_rng(&mut seeded_rng));
        }

        // Verify we got multiple different results
        assert!(results.len() > 1, "Expected multiple random results");

        // Verify all results are valid suits
        let valid_suits: HashSet<_> = ["Hearts", "Diamonds", "Clubs", "Spades"]
            .iter()
            .cloned()
            .collect();
        assert!(results.is_subset(&valid_suits), "Got invalid suit");
    }

    #[test]
    fn get_rand_rank_returns_valid_rank_with_seeded_rng() {
        let mut seeded_rng = StdRng::seed_from_u64(42); // Deterministic seed
        let mut results = HashSet::new();

        // Run multiple times to collect different results
        for _ in 0..30 {
            results.insert(get_rand_rank_with_rng(&mut seeded_rng));
        }

        // Verify we got multiple different results
        assert!(results.len() > 1, "Expected multiple random results");

        // Verify all results are valid ranks
        let valid_ranks: HashSet<_> = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ]
        .iter()
        .cloned()
        .collect();
        assert!(results.is_subset(&valid_ranks), "Got invalid rank");
    }

    #[test]
    fn get_rand_rank_distributes_values_evenly() {
        let mut seeded_rng = StdRng::seed_from_u64(100);
        let mut rank_counts = std::collections::HashMap::new();

        // Generate a large number of ranks to check distribution
        const ITERATIONS: usize = 1000;
        for _ in 0..ITERATIONS {
            let rank = get_rand_rank_with_rng(&mut seeded_rng);
            *rank_counts.entry(rank).or_insert(0) += 1;
        }

        // Check that all 13 ranks appear in the distribution
        assert_eq!(
            rank_counts.len(),
            13,
            "Should have all 13 ranks represented"
        );

        // Each rank should appear approximately 1000/13 ≈ 77 times
        // Allow for some statistical variance (50% margin)
        for count in rank_counts.values() {
            assert!(*count > 30, "Each rank should appear multiple times");
            assert!(*count < 120, "No rank should be overly represented");
        }
    }

    #[test]
    fn get_rand_suite_distributes_values_evenly() {
        let mut seeded_rng = StdRng::seed_from_u64(100);
        let mut suite_counts = std::collections::HashMap::new();

        // Generate a large number of suites to check distribution
        const ITERATIONS: usize = 1000;
        for _ in 0..ITERATIONS {
            let suite = get_rand_suite_with_rng(&mut seeded_rng);
            *suite_counts.entry(suite).or_insert(0) += 1;
        }

        // Check that all 4 suites appear in the distribution
        assert_eq!(
            suite_counts.len(),
            4,
            "Should have all 4 suites represented"
        );

        // Each suite should appear approximately 1000/4 = 250 times
        // Allow for some statistical variance (40% margin)
        for count in suite_counts.values() {
            assert!(*count > 150, "Each suite should appear multiple times");
            assert!(*count < 350, "No suite should be overly represented");
        }
    }
}
