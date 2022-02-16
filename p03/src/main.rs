/*
 * Prompt:
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 *
 * Solution:
 * Loop from 1 to the square root of the target number. Divide number by the
 * whatever values you enounter that it evenly divides into, these will be its
 * prime factors. By starting at the smallest number and continuing until reach the
 * top, you guarantee that you will reduce the target number to its larges prime
 * factor. This algorithm runs in O(sqrt(N)) where n is the target. This is the
 * minimum possible solution as the prime factor could be its square root and
 * must consider up until and including its square root.
 */

mod test;

fn main() {
    let target: u64 = 600_851_475_143;
    let sol: u64 = largest_prime_factor(target);
    println!("The largest prime factor of {} is: {}", target, sol);
}

fn largest_prime_factor(target: u64) -> u64 {
    let sol_sqrt: u64 = (target as f64).sqrt() as u64;
    let mut sol = target;

    for current_test in 2..=sol_sqrt {
        if sol != current_test && sol % current_test == 0 {
            sol = sol / current_test;
        }
    }

    sol
}
