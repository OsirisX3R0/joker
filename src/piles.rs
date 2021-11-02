use rand::seq::SliceRandom;
use std::vec::Vec;

use crate::cards::Card;

/// An ordered stack of cards that can only be accessed from the top
#[derive(Debug, Clone)]
pub struct Pile {
  cards: Vec<Card>,
}

impl Pile {
  /// Creates a new Pile
  pub fn new() -> Pile {
    Pile { cards: Vec::new() }
  }

  /// Returns the cound of cards in the Pile
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
