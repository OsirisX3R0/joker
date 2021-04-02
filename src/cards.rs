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

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
    self.rank == other.rank && self.suit == other.suit
  }
}

impl Card {
  pub fn from(abbr: &str) -> Card {
    let card: Vec<&str> = abbr.split("").collect();
    let filtered = card.iter().filter(|&&x| x != "").collect::<Vec<&&str>>();

    let rank = filtered[0];
    let suit = filtered[1];
    // println!("{} {:?} {} {}", abbr, filtered, rank, suit);
    Card {
      rank: Rank::from(rank),
      suit: Suit::from(suit),
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
  fn should_create_ace_of_spades() {
    let card = Card::from("AS");

    let aos = Card {
      rank: Rank::ACE,
      suit: Suit::SPADES,
    };

    assert_eq!(card == aos, true)
  }

  #[test]
  #[should_panic]
  fn should_fail_invalid_card() {
    let card = Card::from("XX");

    println!("{}", card);
  }
}
