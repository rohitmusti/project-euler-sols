/* Prompt:
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 *           a2 + b2 = c2
 * For example, 32 + 42 = 9 + 16 = 25 = 52.  There exists exactly
 * one Pythagorean triplet for which a + b + c = 1000.
 *
 * Find the product abc.
 *
 * Approach:
 * Right now this is brute forced, it feels pretty gross
 * I am sure there is a faster implementation, but this works and runs very quickly
 */

mod test;

fn main() {
    let target_sum = 1000;
    let sol = pythagorean_triple_product(target_sum);
    println!(
        "The product of the pythagorean triple that sums to {}: {}",
        target_sum, sol
    );
}

fn pythagorean_triple_product(target_sum: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 1..(target_sum - 2) {
        for j in (i + 1)..(target_sum - 2) {
            for k in (j + 1)..(target_sum - 2) {
                let candidate = i + j + k;
                if candidate < target_sum {
                    continue;
                } else if candidate > target_sum {
                    break;
                }
                if i.pow(2) + j.pow(2) == k.pow(2) {
                    result = i * j * k;
                }
            }
        }
    }
    result
}
