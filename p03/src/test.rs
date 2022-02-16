#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        use crate::largest_prime_factor;
        assert_eq!(largest_prime_factor(13_195), 29);
    }
}
