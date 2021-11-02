use std::vec::Vec;

use crate::cards::Card;

pub struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  /// Creates a new Pile
  pub fn new() -> Hand {
    Hand { cards: Vec::new() }
  }

  /// Returns the cound of cards in the Hand
  pub fn count(&self) -> usize {
    self.cards.len()
  }

  /// Adds a new card to the Hand
  pub fn add(&mut self, card: Card) {
    self.cards.push(card)
  }

  /// Removes a card from the Hand
  pub fn remove(&mut self, index: usize) -> Card {
    self.cards.remove(index)
  }

  /// Discards all cards from the Hand
  pub fn discard(&mut self) -> Vec<Card> {
    let clone = self.cards.clone();
    self.cards.clear();

    clone
  }
}
