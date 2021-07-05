pub mod error;
pub mod packet_parsing;
pub mod validators;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
