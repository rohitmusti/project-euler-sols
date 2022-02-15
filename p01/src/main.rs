/*
 * Prompt:
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn main() {
    let target: u32 = 1000;
    let sol: u32 = all_divis_3_5(target).iter().sum();
    println!(
        "The sum of all natural numbers beneath {} is: {}",
        target, sol
    );
}

pub fn all_divis_3_5(val: u32) -> Vec<u32> {
    let mut sol: Vec<u32> = Vec::new();
    for val in 3..val {
        if val % 3 == 0 || val % 5 == 0 {
            sol.push(val);
        }
    }
    sol
}

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
