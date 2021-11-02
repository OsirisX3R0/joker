use joker::Rank;

fn main() {
  let two = Rank::from("2").unwrap();
  let three = Rank::from("3").unwrap();
  let four = Rank::from("4").unwrap();
  let five = Rank::from("5").unwrap();
  let six = Rank::from("6").unwrap();
  let seven = Rank::from("7").unwrap();
  let eight = Rank::from("8").unwrap();
  let nine = Rank::from("9").unwrap();
  let ten = Rank::from("T").unwrap();
  let jack = Rank::from("J").unwrap();
  let queen = Rank::from("Q").unwrap();
  let king = Rank::from("K").unwrap();
  let ace = Rank::from("A").unwrap();

  println!("{}", two);
  println!("{}", three);
  println!("{}", four);
  println!("{}", five);
  println!("{}", six);
  println!("{}", seven);
  println!("{}", eight);
  println!("{}", nine);
  println!("{}", ten);
  println!("{}", jack);
  println!("{}", queen);
  println!("{}", king);
  println!("{}", ace);
  println!("{}", two.to_string());
  println!("{}", three.to_string());
  println!("{}", four.to_string());
  println!("{}", five.to_string());
  println!("{}", six.to_string());
  println!("{}", seven.to_string());
  println!("{}", eight.to_string());
  println!("{}", nine.to_string());
  println!("{}", ten.to_string());
  println!("{}", jack.to_string());
  println!("{}", queen.to_string());
  println!("{}", king.to_string());
  println!("{}", ace.to_string());
}
