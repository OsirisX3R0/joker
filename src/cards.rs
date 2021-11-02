use std::fmt::{Display, Error, Formatter};

use crate::ranks::Rank;
use crate::suits::Suit;

/// A card, composed of a suit and optional rank (Joker)
#[derive(Clone, Copy)]
pub struct Card {
  rank: Option<Rank>,
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

  fn ne(&self, other: &Self) -> bool {
    self.rank != other.rank || self.suit != other.suit
  }
}

impl Card {
  /// Creates a card from a two-character string
  pub fn from(abbr: &str) -> Result<Card, String> {
    if abbr.chars().count() == 2 {
      let r = &abbr[..1];
      let s = &abbr[1..];

      let rank = match Rank::from(r) {
        Ok(rank) => rank,
        Err(err) => return Err(err),
      };
      let suit = match Suit::from(s) {
        Ok(suit) => suit,
        Err(err) => return Err(err),
      };
      Ok(Card {
        rank: Some(rank),
        suit: suit,
      })
    } else if abbr.chars().count() == 1 {
      if abbr != "J" {
        Err(format!("Invalid rank/suit"))
      } else {
        let suit = match Suit::from(abbr) {
          Ok(suit) => suit,
          Err(err) => return Err(err),
        };
        Ok(Card {
          rank: None,
          suit: suit,
        })
      }
    } else {
      Err(format!("Invalid amount of characters"))
    }
  }

  /// Translates a card into a humanized string
  pub fn to_string(&self) -> String {
    let rank_str = match &self.rank {
      Some(rank) => rank.to_string(),
      None => "",
    };
    let suit_str = &self.suit.to_string();
    format!("{} of {}", rank_str, suit_str)
  }
}

#[cfg(test)]
mod card_tests {
  use super::*;

  #[test]
  fn should_create_ace_of_spades() {
    let card = Card::from("AS").unwrap();

    let aos = Card {
      rank: Some(Rank::ACE),
      suit: Suit::SPADES,
    };

    assert_eq!(card == aos, true)
  }

  #[test]
  fn should_create_four_of_clubs() {
    let card = Card::from("4C").unwrap();

    let foc = Card {
      rank: Some(Rank::FOUR),
      suit: Suit::CLUBS,
    };

    assert_eq!(card == foc, true)
  }

  #[test]
  fn should_create_ten_of_diamonds() {
    let card = Card::from("TD").unwrap();

    let tod = Card {
      rank: Some(Rank::TEN),
      suit: Suit::DIAMONDS,
    };

    assert_eq!(card == tod, true)
  }

  #[test]
  fn should_create_joker() {
    let card = Card::from("J").unwrap();

    let tod = Card {
      rank: None,
      suit: Suit::JOKER,
    };

    assert_eq!(card == tod, true)
  }

  #[test]
  #[should_panic]
  fn should_fail_invalid_card() {
    let card = Card::from("XX").unwrap();

    println!("{}", card);
  }
}
