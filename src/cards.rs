use std::fmt::{format, Display, Formatter};

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
  pub fn new(abbr: &mut str) -> Card {
    let card: Vec<&str> = abbr.split("");

    Card {
      rank: card[0],
      suit: card[1],
    }
  }

  pub fn to_string(&self) -> &str {
    !format!("{} of {}" self.rank.to_string(), self.suit.to_string())
  }
}

#[cfg(test)]
mod card_tests {
  use super::*;

  // #[test]
  // fn should_create_ace_of_spades() {
  //   let card = Card::from("AS");

  //   assert_eq!
  // }

  #[test]
  #[should_panic]
  fn should_fail_invalid_card() {
    let card = Card::from("XX");

    println!("{}", card);
  }
}
