/*
 * Prompt:
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *
 * Pretty straight forward algorithm: loop through all numbers, check if they are divisible by 3 or
 * 5, append if so, otherwise ignore. The minimum possible run time for solving this problem is
 * O(n) where n is the target number and this algorithm runs in that time.
 *
 */

mod test;

fn main() {
    let target: u32 = 1000;
    let sol: u32 = all_divis_3_5(target).iter().sum();
    println!(
        "The sum of all natural numbers beneath {} is: {}",
        target, sol
    );
}

fn all_divis_3_5(val: u32) -> Vec<u32> {
    let mut sol: Vec<u32> = Vec::new();
    for val in 3..val {
        if val % 3 == 0 || val % 5 == 0 {
            sol.push(val);
        }
    }
    sol
}
