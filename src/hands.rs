use std::vec::Vec;

use crate::cards::Card;

pub struct Hand(Vec<Card>);

impl Hand {
  pub fn new() -> Hand {
    Hand(Vec::new())
  }

  pub fn add(&mut self, card: Card) {
    self.0.push(card)
  }

  pub fn play(&mut self, index: usize) -> Card {
    self.0.remove(index)
  }
}
