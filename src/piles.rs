// use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Vec;

use crate::cards::Card;

pub struct Pile(Vec<Card>);

impl Pile {
  fn new() -> Pile {
    Pile(Vec::new())
  }

  fn shuffle(&mut self) {
    // self.0 = self.0.collect().shuffle(&mut thread_rng())
  }

  fn discard(&mut self, card: Card) {
    self.0.push(card)
  }

  fn draw(&mut self) -> Card {
    self.0.remove(0)
  }
}
