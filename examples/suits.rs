use joker::Suit;

fn main() {
  let clubs = Suit::from("C").unwrap();
  let diamonds = Suit::from("D").unwrap();
  let hearts = Suit::from("H").unwrap();
  let spades = Suit::from("S").unwrap();
  let joker = Suit::from("J").unwrap();

  println!("{}", clubs);
  println!("{}", diamonds);
  println!("{}", hearts);
  println!("{}", spades);
  println!("{}", joker);
  println!("{}", clubs.to_string());
  println!("{}", diamonds.to_string());
  println!("{}", hearts.to_string());
  println!("{}", spades.to_string());
  println!("{}", joker.to_string());
}
