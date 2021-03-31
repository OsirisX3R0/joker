use std::fmt::{Display, Error, Formatter};

/// All possible ranks for cards
#[derive(Debug)]
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
  JOKER,
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
        Rank::TEN => "10",
        Rank::JACK => "J",
        Rank::QUEEN => "Q",
        Rank::KING => "K",
        Rank::ACE => "A",
        Rank::JOKER => "JK",
      }
    )
  }
}

impl Rank {
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
      Rank::JOKER => "Joker",
    }
  }
}
