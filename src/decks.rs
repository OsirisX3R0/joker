use strum::IntoEnumIterator;

use crate::cards::Card;
use crate::piles::Pile;
use crate::ranks::Rank;
use crate::suits::Suit;

pub struct Deck {
  draw_pile: Pile,
  discard_pile: Pile,
}

impl Deck {
  /// Generates a new Deck setup
  pub fn new(jokers: bool, shuffle: bool) -> Deck {
    Deck {
      draw_pile: Deck::generate_standard_deck(jokers, shuffle),
      discard_pile: Pile::new(),
    }
  }

  /// Generates a standard 52-card Pile
  pub fn generate_standard_deck(jokers: bool, shuffle: bool) -> Pile {
    let mut pile = Pile::new();
    for suit in Suit::iter() {
      match suit {
        Suit::JOKER => {
          if jokers {
            continue;
          }
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
    if shuffle {
      pile.shuffle()
    } else {
      pile
    }
  }
}

#[cfg(test)]
mod deck_tests {
  use super::*;

  #[test]
  fn should_build_standard() {
    let pile = Deck::generate_standard_deck(false, false);

    assert_eq!(pile.count(), 52)
  }

  #[test]
  fn should_build_standard_with_jokers() {
    let pile = Deck::generate_standard_deck(true, false);

    assert_eq!(pile.count(), 54)
  }
}
