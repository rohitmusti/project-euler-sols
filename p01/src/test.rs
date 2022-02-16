#[cfg(test)]
mod tests {
    #[test]
    fn ten_test() {
        use crate::all_divis_3_5;
        let sol: Vec<u32> = vec![3, 5, 6, 9];
        assert_eq!(sol, all_divis_3_5(10));
    }

    #[test]
    fn fifteen_test() {
        use crate::all_divis_3_5;
        let sol: Vec<u32> = vec![3, 5, 6, 9, 10, 12];
        assert_eq!(sol, all_divis_3_5(15));
    }
}
