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

impl Suit {
  pub fn from(abbr: &str) -> Suit {
    let suit = match abbr {
      "C" => Ok(Suit::CLUBS),
      "S" => Ok(Suit::SPADES),
      "H" => Ok(Suit::HEARTS),
      "D" => Ok(Suit::DIAMONDS),
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
