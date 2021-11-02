use std::fmt::{Display, Error, Formatter};

/// All possible ranks for cards (Two through Ace, lowest to highest)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rank {
  TWO,
  THREE,
  FOUR,
  FIVE,
  SIX,
  SEVEN,
  EIGHT,
  NINE,
  TEN,
  JACK,
  QUEEN,
  KING,
  ACE,
}

impl Display for Rank {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(
      f,
      "{}",
      match &self {
        Rank::TWO => "2",
        Rank::THREE => "3",
        Rank::FOUR => "4",
        Rank::FIVE => "5",
        Rank::SIX => "6",
        Rank::SEVEN => "7",
        Rank::EIGHT => "8",
        Rank::NINE => "9",
        Rank::TEN => "T",
        Rank::JACK => "J",
        Rank::QUEEN => "Q",
        Rank::KING => "K",
        Rank::ACE => "A",
      }
    )
  }
}

impl Rank {
  /// Creates a rank from a character
  pub fn from(abbr: &str) -> Result<Rank, String> {
    match abbr {
      "2" => Ok(Rank::TWO),
      "3" => Ok(Rank::THREE),
      "4" => Ok(Rank::FOUR),
      "5" => Ok(Rank::FIVE),
      "6" => Ok(Rank::SIX),
      "7" => Ok(Rank::SEVEN),
      "8" => Ok(Rank::EIGHT),
      "9" => Ok(Rank::NINE),
      "T" => Ok(Rank::TEN),
      "J" => Ok(Rank::JACK),
      "Q" => Ok(Rank::QUEEN),
      "K" => Ok(Rank::KING),
      "A" => Ok(Rank::ACE),
      &_ => Err(format!("{} is not a valid rank", abbr)),
    }
  }

  /// Translates a rank into a humanized string
  pub fn to_string(&self) -> &str {
    match &self {
      Rank::TWO => "Two",
      Rank::THREE => "Three",
      Rank::FOUR => "Four",
      Rank::FIVE => "Five",
      Rank::SIX => "Six",
      Rank::SEVEN => "Seven",
      Rank::EIGHT => "Eight",
      Rank::NINE => "Nine",
      Rank::TEN => "Ten",
      Rank::JACK => "Jack",
      Rank::QUEEN => "Queen",
      Rank::KING => "King",
      Rank::ACE => "Ace",
    }
  }
}

#[cfg(test)]
mod rank_tests {
  use super::*;

  #[test]
  fn should_create_two_rank() {
    let rank = Rank::from("2").unwrap();

    assert_eq!(Rank::TWO, rank);
  }

  #[test]
  fn should_create_queen_rank() {
    let rank = Rank::from("Q").unwrap();

    assert_eq!(Rank::QUEEN, rank);
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
