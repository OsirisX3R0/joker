use regex::Regex;
use std::fmt::{Display, Error, Formatter};
use strum_macros::EnumIter;

/// All possible ranks for cards (Two through Ace, lowest to highest)
#[derive(Debug, Clone, PartialEq, PartialOrd, EnumIter)]
pub enum Rank {
  NUMBER(u8),
  FACE(String),
}

impl Display for Rank {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(
      f,
      "{}",
      match &self {
        Rank::NUMBER(num) => num.to_string(),
        Rank::FACE(str) => String::from(str),
      }
    )
  }
}

impl Rank {
  /// Creates a rank from a character
  pub fn from(abbr: &str) -> Result<Rank, String> {
    let is_number = Regex::new(r"[\dT]").unwrap();
    let is_face = Regex::new(r"[JQKA]").unwrap();

    if is_number.is_match(abbr) {
      return match abbr {
        "T" => Ok(Rank::NUMBER(10)),
        other => Ok(Rank::NUMBER(other.parse::<u8>().unwrap())),
      };
    } else if is_face.is_match(abbr) {
      Ok(Rank::FACE(String::from(abbr)))
    } else {
      Err(format!("{} is not a valid rank", abbr))
    }
  }

  /// Translates a rank into a humanized string
  pub fn to_string(self) -> String {
    let jack = String::from("J");
    let queen = String::from("Q");
    let king = String::from("K");
    let ace = String::from("A");

    let string = match self {
      Rank::NUMBER(2) => "Two",
      Rank::NUMBER(3) => "Three",
      Rank::NUMBER(4) => "Four",
      Rank::NUMBER(5) => "Five",
      Rank::NUMBER(6) => "Six",
      Rank::NUMBER(7) => "Seven",
      Rank::NUMBER(8) => "Eight",
      Rank::NUMBER(9) => "Nine",
      Rank::NUMBER(10) => "Ten",
      Rank::FACE(jack) => "Jack",
      Rank::FACE(queen) => "Queen",
      Rank::FACE(king) => "King",
      Rank::FACE(ace) => "Ace",
      _ => "",
    };

    String::from(string)
  }
}

#[cfg(test)]
mod rank_tests {
  use super::*;

  #[test]
  fn should_create_two_rank() {
    let rank = Rank::from("2").unwrap();

    assert_eq!(Rank::NUMBER(2), rank);
  }

  #[test]
  fn should_create_queen_rank() {
    let rank = Rank::from("Q").unwrap();

    assert_eq!(Rank::FACE(String::from("Q")), rank);
  }

  #[test]
  #[should_panic]
  fn should_fail_invalid_rank() {
    let rank = Rank::from("X").unwrap();

    println!("{}", rank);
  }

  #[test]
  fn should_be_less_than() {
    let ten = Rank::from("T").unwrap();
    let jack = Rank::from("J").unwrap();

    assert_eq!(ten < jack, true)
  }

  #[test]
  fn should_be_greater_than() {
    let queen = Rank::from("Q").unwrap();
    let four = Rank::from("4").unwrap();
    assert_eq!(queen > four, true)
  }
}
