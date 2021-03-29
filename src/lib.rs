mod ranks;
mod suits;

pub use ranks::Ranks;
pub use suits::Suits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
