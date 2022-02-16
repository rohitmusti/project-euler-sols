#[cfg(test)]
mod tests {
    #[test]
    fn ten_test() {
        use crate::even_fib_under_lim;
        let sol: Vec<u32> = vec![2, 8];
        assert_eq!(even_fib_under_lim(10), sol);
    }

    #[test]
    fn forty_test() {
        use crate::even_fib_under_lim;
        let sol: Vec<u32> = vec![2, 8, 34];
        assert_eq!(even_fib_under_lim(40), sol);
    }
}
