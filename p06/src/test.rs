#[cfg(test)]
mod tests {
    #[test]
    fn sum_of_squares_test() {
        use crate::sum_of_squares;
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn square_of_sums_test() {
        use crate::squares_of_sum;
        assert_eq!(squares_of_sum(10), 3025);
    }

#[test]
    fn diff_test() {
        use crate::diff_square_of_sum_of;
        assert_eq!(diff_square_of_sum_of(10), 2640);
    }
}

