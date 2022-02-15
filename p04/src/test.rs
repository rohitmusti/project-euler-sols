#[cfg(test)]
pub mod tests {
    #[test]
    fn palindrome_tests_true() {
        use crate::is_palindrome;
        // even num digits and true
        assert!(is_palindrome(9009));
        // odd num digits and true
        assert!(is_palindrome(123321));
    }

    #[test]
    fn palindrome_tests_false() {
        use crate::is_palindrome;
        // even num digits and false
        assert!(!is_palindrome(9008));
        // odd num digits and false
        assert!(!is_palindrome(900));
    }
}
