use joker::Card;

fn main() {
  let card1 = Card::from("AS");
  let card2 = Card::from("AZ");
  let card3 = Card::from("XS");

  println!("{}", card1);
  println!("{}", card2);
  println!("{}", card3);
}
