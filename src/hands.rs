use std::vec::Vec;

use crate::cards::Card;

pub struct Hand(Vec<Card>);

impl Hand {
  fn new() -> Hand {
    Hand(Vec::new())
  }

  fn add(&mut self, card: Card) {
    self.0.push(card)
  }

  fn play(&mut self, index: usize) -> Card {
    self.0.remove(index)
  }
}
