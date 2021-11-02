use rand::seq::SliceRandom;
use std::vec::Vec;

use crate::cards::Card;

/// An ordered stack of cards that can only be accessed from the top
pub struct Pile {
  cards: Vec<Card>,
}

impl Pile {
  /// Creates a new Pile
  pub fn new() -> Pile {
    Pile { cards: Vec::new() }
  }

  /// Shuffles the cards contained in the pile
  pub fn shuffle(&mut self) {
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
  }

  /// Add a card to the top of the pile
  pub fn discard(&mut self, card: Card) {
    self.cards.insert(0, card)
  }

  /// Take a card from the top of the pile
  pub fn draw(&mut self) -> Card {
    self.cards.remove(0)
  }
}
