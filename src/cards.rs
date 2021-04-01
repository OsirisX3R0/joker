use std::fmt::{Display, Error, Formatter};

use crate::ranks::Rank;
use crate::suits::Suit;

pub struct Card {
  rank: Rank,
  suit: Suit,
}

impl Display for Card {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", &self.to_string())
  }
}

// TODO: Implement PartialEq

impl Card {
  pub fn from(abbr: &str) -> Card {
    let card: Vec<&str> = abbr.split("").collect();

    Card {
      rank: Rank::from(card[0]),
      suit: Suit::from(card[1]),
    }
  }

  pub fn to_string(&self) -> String {
    format!("{} of {}", &self.rank.to_string(), &self.suit.to_string())
  }
}

#[cfg(test)]
mod card_tests {
  use super::*;
  #[test]
  #[should_panic]
  fn should_fail_invalid_card() {
    let card = Card::from("XX");

    println!("{}", card);
  }
}
