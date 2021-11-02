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

  /// Adds a new card to the hand
  pub fn add(&mut self, card: Card) {
    self.cards.push(card)
  }

  /// Removes a card from the hand
  pub fn remove(&mut self, index: usize) -> Card {
    self.cards.remove(index)
  }

  /// Discards all cards from the hand
  pub fn discard(&mut self) -> Vec<Card> {
    let clone = self.cards.clone();
    self.cards.clear();

    clone
  }
}
