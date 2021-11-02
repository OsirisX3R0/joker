use strum::IntoEnumIterator;

use crate::cards::Card;
use crate::piles::Pile;
use crate::ranks::Rank;
use crate::suits::Suit;

/// Generates a standard 52-card Pile with no jokers
pub fn standard_no_jokers(shuffle: Option<bool>) -> Pile {
  let mut pile = Pile::new();

  for suit in Suit::iter() {
    match suit {
      Suit::JOKER => continue,
      _ => {
        for rank in Rank::iter() {
          let card_str = format!("{}{}", rank, suit);
          let card = Card::from(&card_str[..]).unwrap();
          pile.stack(card);
        }
      }
    };
  }

  match shuffle {
    Some(shuffle) => {
      if shuffle {
        pile.shuffle()
      } else {
        pile
      }
    }
    None => pile,
  }
}

/// Generates a standard 52-card Pile with jokers included
pub fn standard_with_jokers(shuffle: Option<bool>) -> Pile {
  let mut pile = Pile::new();

  for suit in Suit::iter() {
    match suit {
      Suit::JOKER => {
        pile.stack(Card::from("J").unwrap());
        pile.stack(Card::from("J").unwrap());
      }
      _ => {
        for rank in Rank::iter() {
          let card_str = format!("{}{}", rank, suit);
          let card = Card::from(&card_str[..]).unwrap();
          pile.stack(card);
        }
      }
    };
  }

  match shuffle {
    Some(shuffle) => {
      if shuffle {
        pile.shuffle()
      } else {
        pile
      }
    }
    None => pile,
  }
}

pub struct Deck {
  draw_pile: Pile,
  discard_pile: Pile,
}

impl Deck {
  /// Generates a new Deck setup
  pub fn new(jokers: Option<bool>, shuffle: Option<bool>) -> Deck {
    let shuffle_deck = match shuffle {
      Some(shuffle) => shuffle,
      None => false,
    };

    Deck {
      draw_pile: match jokers {
        Some(jokers) => {
          if jokers {
            standard_with_jokers(Some(shuffle_deck))
          } else {
            standard_no_jokers(Some(shuffle_deck))
          }
        }
        None => standard_no_jokers(Some(shuffle_deck)),
      },
      discard_pile: Pile::new(),
    }
  }
}

#[cfg(test)]
mod deck_tests {
  use super::*;

  #[test]
  fn should_build_standard() {
    let pile = standard_no_jokers(None);

    assert_eq!(pile.count(), 52)
  }

  #[test]
  fn should_build_standard_with_jokers() {
    let pile = standard_with_jokers(None);

    assert_eq!(pile.count(), 54)
  }
}
