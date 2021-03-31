mod ranks;
mod suits;

pub use ranks::Rank;
pub use suits::Suit;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
