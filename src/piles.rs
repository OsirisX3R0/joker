use rand::seq::SliceRandom;
use std::vec::Vec;

use crate::cards::Card;

pub struct Pile(Vec<Card>);

// impl Iterator for Pile {
//   type Item = Card;

//   fn next(&mut self) -> Option<Self::Item> {}
// }

impl Pile {
  pub fn new() -> Pile {
    Pile(Vec::new())
  }

  pub fn shuffle(&mut self) {
    let mut shuffled = Vec::new();
    let mut rng = rand::thread_rng();
    let mut currentIndex = self.0.len();
    let mut indexes: Vec<usize> = (0..currentIndex).collect();
    let mut randomIndex: usize;

    while currentIndex != 0 {
      indexes.shuffle(&mut rng);

      for i in indexes {
        shuffled.push(&self.0[i])
      }
    }

    self.0 = shuffled;
  }

  pub fn discard(&mut self, card: Card) {
    self.0.push(card)
  }

  pub fn draw(&mut self) -> Card {
    self.0.remove(0)
  }
}
