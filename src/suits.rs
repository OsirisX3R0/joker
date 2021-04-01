use std::fmt::{Display, Error, Formatter};

/// All possible suits for cards
#[derive(Debug)]
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
        _ => "", // Suit::JOKER => "J",
      }
    )
  }
}

impl PartialEq for Suit {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Suit::CLUBS => match other {
        Suit::CLUBS => true,
        &_ => false,
      },
      Suit::SPADES => match other {
        Suit::SPADES => true,
        &_ => false,
      },
      Suit::HEARTS => match other {
        Suit::HEARTS => true,
        &_ => false,
      },
      Suit::DIAMONDS => match other {
        Suit::DIAMONDS => true,
        &_ => false,
      },
      Suit::JOKER => match other {
        Suit::JOKER => true,
        &_ => false,
      },
    }
  }
}

impl Suit {
  pub fn from(abbr: &str) -> Suit {
    let suit = match abbr {
      "C" => Ok(Suit::CLUBS),
      "S" => Ok(Suit::SPADES),
      "H" => Ok(Suit::HEARTS),
      "D" => Ok(Suit::DIAMONDS),
      "" => Ok(Suit::JOKER),
      &_ => Err(format!("{} not a valid suit", abbr)),
    };

    suit.unwrap()
  }
  /// Translates a suit into a humanized string
  pub fn to_string(&self) -> &str {
    match &self {
      Suit::CLUBS => "Clubs",
      Suit::SPADES => "Spades",
      Suit::HEARTS => "Hearts",
      Suit::DIAMONDS => "Diamonds",
      &_ => "", // Suit::JOKER => "J",
    }
  }
}

#[cfg(test)]
mod suit_tests {
  use super::*;

  #[test]
  fn should_create_clubs_suit() {
    let suit = Suit::from("C");
    assert_eq!(suit == Suit::CLUBS, true);
  }

  #[test]
  fn should_create_spades_suit() {
    let suit = Suit::from("S");
    assert_eq!(suit == Suit::SPADES, true);
  }

  #[test]
  fn should_create_hearts_suit() {
    let suit = Suit::from("H");
    assert_eq!(suit == Suit::HEARTS, true);
  }

  #[test]
  fn should_create_diamond_suit() {
    let suit = Suit::from("D");
    assert_eq!(suit == Suit::DIAMONDS, true);
  }

  #[test]
  fn should_create_joker_suit() {
    let suit = Suit::from("");
    assert_eq!(suit == Suit::JOKER, true);
  }

  #[test]
  #[should_panic]
  fn should_fail_invalid_suit() {
    let suit = Suit::from("X");
    println!("{}", suit);
  }
}
