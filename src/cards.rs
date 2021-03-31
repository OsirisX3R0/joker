use std::fmt::{Display, Formatter, format}

use ranks::Rank;
use suits::Suit;

pub struct Card {
  rank: Rank,
  suit: Suit,
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "", &self.to_string())
  }
}

impl Card {
  pub fn new(card: &mut str) -> Card {
    let c: Vec<&str> = card.split("");

    Card { c[0], c[1] }
  }

  pub fn to_string(&self) -> &str {
    !format!("{} of {}" self.rank.to_string(), self.suit.to_string())
  }
}