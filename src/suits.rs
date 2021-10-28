use std::fmt::{Display, Error, Formatter};

/// All possible suits for cards
#[derive(Debug, PartialEq)]
pub enum Suit {
  CLUBS,
  SPADES,
  HEARTS,
  DIAMONDS,
  JOKER,
}

impl Display for Suit {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(
      f,
      "{}",
      match &self {
        Suit::CLUBS => "C",
        Suit::SPADES => "S",
        Suit::HEARTS => "H",
        Suit::DIAMONDS => "D",
        Suit::JOKER => "J",
      }
    )
  }
}

impl Suit {
  pub fn from(abbr: &str) -> Result<Suit, String> {
    let suit = match abbr {
      "C" => Ok(Suit::CLUBS),
      "S" => Ok(Suit::SPADES),
      "H" => Ok(Suit::HEARTS),
      "D" => Ok(Suit::DIAMONDS),
      "J" => Ok(Suit::JOKER),
      &_ => Err(format!("{} is not a valid suit", abbr)),
    };

    suit
  }
  /// Translates a suit into a humanized string
  pub fn to_string(&self) -> &str {
    match &self {
      Suit::CLUBS => "Clubs",
      Suit::SPADES => "Spades",
      Suit::HEARTS => "Hearts",
      Suit::DIAMONDS => "Diamonds",
      Suit::JOKER => "Joker",
    }
  }
}

#[cfg(test)]
mod suit_tests {
  use super::*;

  #[test]
  fn should_create_clubs_suit() {
    let suit = Suit::from("C").unwrap();
    assert_eq!(suit == Suit::CLUBS, true);
  }

  #[test]
  fn should_create_spades_suit() {
    let suit = Suit::from("S").unwrap();
    assert_eq!(suit == Suit::SPADES, true);
  }

  #[test]
  fn should_create_hearts_suit() {
    let suit = Suit::from("H").unwrap();
    assert_eq!(suit == Suit::HEARTS, true);
  }

  #[test]
  fn should_create_diamond_suit() {
    let suit = Suit::from("D").unwrap();
    assert_eq!(suit == Suit::DIAMONDS, true);
  }

  #[test]
  fn should_create_joker_suit() {
    let suit = Suit::from("J").unwrap();
    assert_eq!(suit == Suit::JOKER, true);
  }

  #[test]
  #[should_panic]
  fn should_fail_invalid_suit() {
    let suit = Suit::from("X").unwrap();
    println!("{}", suit);
  }
}
