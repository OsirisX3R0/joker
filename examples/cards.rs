use joker::Card;

fn main() {
  let toc = Card::from("3C").unwrap();
  let tod = Card::from("TD").unwrap();
  let qoh = Card::from("QH").unwrap();
  let aos = Card::from("AS").unwrap();
  let joker = Card::from("J").unwrap();
  // let card4 = Card::from("XS").unwrap(); // panics

  println!("{}", toc);
  println!("{}", tod);
  println!("{}", qoh);
  println!("{}", aos);
  println!("{}", joker);
  // println!("{}", card4);
  println!("{}", toc.to_string());
  println!("{}", tod.to_string());
  println!("{}", qoh.to_string());
  println!("{}", aos.to_string());
  println!("{}", joker.to_string());
  // println!("{}", card4.to_string());
}
