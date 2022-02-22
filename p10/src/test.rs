#[cfg(test)]
mod tests {
    #[test]
    fn all_primes_test() {
        use crate::all_primes;
        assert_eq!(all_primes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn is_prime_test() {
        use crate::is_prime;
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(15), false);

        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(2), true);
    }
}
