//! # Blackjack Card Game
//!
//! This module implements a simple interactive Blackjack card game
//! with standard rules and gameplay mechanics.
//!
//! ## Features
//!
//! - **Card Representation**: Models playing cards with suits and ranks
//! - **Deck Management**: Implements a full 52-card deck with shuffling and dealing
//! - **Hand Evaluation**: Calculates hand values with special Ace handling (1 or 11)
//! - **Game Logic**: Follows standard Blackjack rules for player and dealer actions
//! - **Interactive Play**: Offers players choices to hit or stand during gameplay
//! - **Bust Detection**: Identifies when a hand exceeds 21 points
//! - **Game Outcome**: Determines winners based on final hand values
use rand::seq::SliceRandom;
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suite::Hearts => "Hearts",
                Suite::Diamonds => "Diamonds",
                Suite::Clubs => "Clubs",
                Suite::Spades => "Spades",
            }
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Ace => "Ace",
                Rank::Two => "Two",
                Rank::Three => "Three",
                Rank::Four => "Four",
                Rank::Five => "Five",
                Rank::Six => "Six",
                Rank::Seven => "Seven",
                Rank::Eight => "Eight",
                Rank::Nine => "Nine",
                Rank::Ten => "Ten",
                Rank::Jack => "Jack",
                Rank::Queen => "Queen",
                Rank::King => "King",
            }
        )
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Card {
    suit: Suite,
    value: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in [Suite::Hearts, Suite::Diamonds, Suite::Clubs, Suite::Spades] {
            for value in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card {
                    suit: suit.clone(),
                    value: value.clone(),
                });
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

enum Move {
    Hit,
    Stand,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn evaluate(&self) -> u32 {
        let mut sum = 0;
        let mut ace_count = 0;

        // First pass: Count all non-Ace cards and track number of Aces
        for card in &self.cards {
            match card.value {
                Rank::Ace => ace_count += 1,
                Rank::Two => sum += 2,
                Rank::Three => sum += 3,
                Rank::Four => sum += 4,
                Rank::Five => sum += 5,
                Rank::Six => sum += 6,
                Rank::Seven => sum += 7,
                Rank::Eight => sum += 8,
                Rank::Nine => sum += 9,
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => sum += 10,
            }
        }

        // Second pass: Add Aces as 11 when possible, otherwise as 1
        for _ in 0..ace_count {
            if sum + 11 <= 21 {
                sum += 11;
            } else {
                sum += 1;
            }
        }

        // Final check: If we're still over 21 and have used Aces as 11, convert them back to 1
        while sum > 21 && ace_count > 0 {
            sum -= 10; // Convert one Ace from 11 to 1 (subtract 10)
            ace_count -= 1;
        }

        sum
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for card in &self.cards {
            writeln!(f, "\t{}", card)?;
        }
        Ok(())
    }
}

fn prompt_for_move() -> Move {
    loop {
        println!("Do you want to hit(H) or stand(S)?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "H" => return Move::Hit,
            "S" => return Move::Stand,
            _ => println!("Invalid input. Please enter 'H' or 'S'."),
        }
    }
}

fn main() {
    const BLACKJACK: u32 = 21;

    let mut deck = Deck::new();
    deck.shuffle();

    let mut player_hand = Hand::new();
    player_hand.add_card(deck.deal().unwrap());
    player_hand.add_card(deck.deal().unwrap());

    loop {
        println!("Your hand: \n{}", player_hand);

        match prompt_for_move() {
            Move::Stand => {
                let mut dealer_hand = Hand::new();
                dealer_hand.add_card(deck.deal().unwrap());
                dealer_hand.add_card(deck.deal().unwrap());
                println!("Dealer hand: \n{}", dealer_hand);

                let player_score = player_hand.evaluate();
                let dealer_score = dealer_hand.evaluate();
                match player_score.cmp(&dealer_score) {
                    std::cmp::Ordering::Less => println!("You lose!"),
                    std::cmp::Ordering::Equal => println!("It's a tie!"),
                    std::cmp::Ordering::Greater => println!("You win!"),
                }
                break;
            }
            Move::Hit => {
                if let Some(card) = deck.deal() {
                    player_hand.add_card(card);
                    println!("You drew: {}", player_hand.cards.last().unwrap());
                    if player_hand.evaluate() > BLACKJACK {
                        println!("Bust! Your hand is over 21.");
                        break;
                    }
                } else {
                    println!("No more cards in the deck.");
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_deck_with_correct_number_of_cards() {
        let deck = Deck::new();
        // 13 cards per suit (2-14) * 4 suits = 52 cards
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn new_creates_deck_with_all_values_for_each_suit() {
        let deck = Deck::new();
        let seen_cards = deck.cards.iter().collect::<std::collections::HashSet<_>>();

        for suit in [Suite::Hearts, Suite::Diamonds, Suite::Clubs, Suite::Spades] {
            for value in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                assert!(seen_cards.contains(&&Card {
                    suit: suit.clone(),
                    value: value.clone()
                }));
            }
        }
    }

    #[test]
    fn new_creates_deck_without_duplicates() {
        let mut deck = Deck::new();
        let mut seen_cards = std::collections::HashSet::new();
        while let Some(card) = deck.deal() {
            assert!(!seen_cards.contains(&card));
            seen_cards.insert(card);
        }
    }

    #[test]
    fn evaluate_returns_correct_value_for_empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.evaluate(), 0);
    }

    #[test]
    fn evaluate_calculates_numbered_cards_correctly() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Two,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Three,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::Four,
        });
        assert_eq!(hand.evaluate(), 9);
    }

    #[test]
    fn evaluate_calculates_face_cards_correctly() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Jack,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Queen,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::King,
        });
        assert_eq!(hand.evaluate(), 30);
    }

    #[test]
    fn evaluate_calculates_mixed_cards_correctly() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Two,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Queen,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::Seven,
        });
        assert_eq!(hand.evaluate(), 19);
    }

    #[test]
    fn evaluate_handles_single_ace_as_eleven_when_possible() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Five,
        });
        assert_eq!(hand.evaluate(), 16); // Ace should be 11
    }

    #[test]
    fn evaluate_handles_single_ace_as_one_when_necessary() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Ten,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::Queen,
        });
        assert_eq!(hand.evaluate(), 21); // Ace must be 1 to avoid bust
    }

    #[test]
    fn evaluate_handles_multiple_aces_correctly() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::Nine,
        });

        // First Ace as 11, second Ace as 1: 11 + 1 + 9 = 21
        assert_eq!(hand.evaluate(), 21);
    }

    #[test]
    fn evaluate_handles_all_aces_as_one_when_necessary() {
        let mut hand = Hand::new();
        hand.add_card(Card {
            suit: Suite::Hearts,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Diamonds,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Clubs,
            value: Rank::Ace,
        });
        hand.add_card(Card {
            suit: Suite::Spades,
            value: Rank::King,
        });

        // All Aces must be 1 to avoid bust: 1 + 1 + 1 + 10 = 13
        assert_eq!(hand.evaluate(), 13);
    }
}
