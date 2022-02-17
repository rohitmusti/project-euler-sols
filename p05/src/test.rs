#[cfg(test)]
mod tests {
    #[test]
    fn smalles_divis_test() {
        use crate::smallest_divis_by;
        assert_eq!(smallest_divis_by(10), 2520);
    }

    #[test]
    fn is_prime_test() {
        use crate::is_prime;
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(2), true);
    }

#[test]
    fn prime_factor_max() {
        use crate::prime_factor_count;
        assert_eq!(prime_factor_count(&2, 5), 4);
        assert_eq!(prime_factor_count(&2, 10), 8);
        assert_eq!(prime_factor_count(&2, 8), 8);
    }
}
