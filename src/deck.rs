use std::fmt::{Display, Error, Formatter};
use std::vec::Vec;

use crate::cards::Card;

struct Deck {
  draw_pile: Vec<Card>,
  discard_pile: Vec<Card>,
}

impl Deck {}
