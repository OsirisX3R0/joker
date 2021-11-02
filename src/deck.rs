use std::fmt::{Display, Error, Formatter};
use std::vec::Vec;
use strum::IntoEnumIterator;

use crate::cards::Card;
use crate::piles::Pile;
use crate::ranks::Rank;
use crate::suits::Suit;

/// Generates a standard 52-card Pile with no jokers
pub fn standard_no_jokers() -> Pile {
  let pile = Pile::new();

  for suit in Suit::iter() {
    match suit {
      JOKER => continue,
      &_ => {
        for rank in Rank::iter() {
          pile.stack(Card::from(format!("{}{}", suit, rank)).unwrap());
        }
      }
    };
  }

  pile.shuffle()
}

/// Generates a standard 52-card Pile with jokers included
pub fn standard_with_jokers() -> Pile {
  let pile = Pile::new();

  for suit in Suit::iter() {
    match suit {
      JOKER => {
        pile.stack(Card::from("J").unwrap());
        pile.stack(Card::from("J").unwrap());
      }
      &_ => {
        for rank in Rank::iter() {
          pile.stack(Card::from(format!("{}{}", suit, rank)).unwrap());
        }
      }
    };
  }

  pile.shuffle()
}

pub struct Deck {
  draw_pile: Vec<Card>,
  discard_pile: Vec<Card>,
}

impl Deck {}

#[cfg(test)]
mod deck_tests {
  use super::*;

  should_build_standard() {
    let pile = standard_no_jokers();

    assert_eq!(pile.count(), 52)
  }

  should_build_standard_with_jokers() {
    let pile = standard_with_jokers();

    assert_eq!(pile.count(), 54)
  }
}
