use rand::seq::SliceRandom;
use std::vec::Vec;

use crate::cards::Card;
use crate::decks;

/// An ordered stack of cards that can only be accessed from the top
#[derive(Debug, Clone)]
pub struct Pile {
  cards: Vec<Card>,
}

impl PartialEq for Pile {
  fn eq(&self, other: &Self) -> bool {
    let mut equal = true;

    if self.count() != other.count() {
      return false;
    }

    for (i, card) in self.cards.iter().enumerate() {
      if equal {
        let other_card = other.cards[i];

        equal = *card == other_card
      }
    }

    equal
  }

  fn ne(&self, other: &Self) -> bool {
    let mut equal = false;

    if self.count() == other.count() {
      return false;
    }

    for (i, card) in self.cards.iter().enumerate() {
      if !equal {
        let other_card = other.cards[i];

        equal = *card != other_card
      }
    }

    equal
  }
}

impl Pile {
  /// Creates a new Pile
  pub fn new() -> Pile {
    Pile { cards: Vec::new() }
  }

  /// Returns the count of cards in the Pile
  pub fn count(&self) -> usize {
    self.cards.len()
  }

  /// Shuffles the cards contained in the Pile
  pub fn shuffle(&mut self) -> Pile {
    let mut shuffled = self.cards.clone();
    let mut rng = rand::thread_rng();
    let current_index = self.cards.len();
    let mut indexes: Vec<usize> = (0..current_index).collect();
    shuffled.clear();
    indexes.shuffle(&mut rng);

    for i in indexes {
      shuffled.push(self.cards[i])
    }

    self.cards = shuffled;

    self.clone()
  }

  /// Add a card to the top of the Pile
  pub fn stack(&mut self, card: Card) {
    self.cards.insert(0, card)
  }

  /// Take a card from the top of the Pile
  pub fn take(&mut self) -> Card {
    self.cards.remove(0)
  }

  /// Clears all cards from the Pile
  pub fn clear(&mut self) -> Vec<Card> {
    let clone = self.cards.clone();
    self.cards.clear();

    clone
  }
}

#[cfg(test)]
mod pile_tests {
  use super::*;

  #[test]
  fn should_create_new() {
    let pile = Pile::new();

    assert_eq!(pile, Pile { cards: Vec::new() })
  }

  #[test]
  fn should_be_same() {
    let pile1 = decks::standard_no_jokers(None);
    let pile2 = decks::standard_no_jokers(None);
    assert_eq!(pile1, pile2)
  }

  #[test]
  fn should_shuffle() {
    let pile1 = decks::standard_no_jokers(None);
    let pile2 = decks::standard_no_jokers(None).shuffle();
    assert_ne!(pile1, pile2)
  }
}
