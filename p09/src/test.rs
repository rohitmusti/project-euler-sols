#[cfg(test)]
mod tests {
    #[test]
    fn three_four_five_test() {
        use crate::pythagorean_triple_product;
        let a = 3;
        let b = 4;
        let c = 5;
        assert_eq!(pythagorean_triple_product(a + b + c), a * b * c);
    }
}
