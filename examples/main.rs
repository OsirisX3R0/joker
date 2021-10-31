use joker::Card;

fn main() {
  let card1 = Card::from("D10").unwrap();
  let card2 = Card::from("AS").unwrap();
  let card3 = Card::from("AZ").unwrap();
  let card4 = Card::from("XS").unwrap();

  println!("{}", card1);
  println!("{}", card2);
  println!("{}", card3);
  println!("{}", card4);
}
