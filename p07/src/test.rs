#[cfg(test)]
mod tests {
    #[test]
    fn is_prime_test() {
        use crate::is_prime;
        assert_eq!(is_prime(&10), false);
        assert_eq!(is_prime(&9), false);
        assert_eq!(is_prime(&13), true);
        assert_eq!(is_prime(&2), true);
    }

    #[test]
    fn nth_prime_test() {
        use crate::find_nth_prime;
        assert_eq!(find_nth_prime(6), 13);
        assert_eq!(find_nth_prime(7), 17);
    }
}
