/*
 * Prompt:
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */

fn main() {
    let target: u64 = 600_851_475_143;
    let sol: u64 = largest_prime_factor(target);
    println!("The largest prime factor of {} is: {}", target, sol);
}

pub fn largest_prime_factor(target: u64) -> u64 {
    let sol_sqrt: u64 = (target as f64).sqrt() as u64;
    let mut sol = target;

    for current_test in 2..=sol_sqrt {
        if sol != current_test && sol % current_test == 0 {
            sol = sol / current_test;
        }
    }

    sol
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        use crate::largest_prime_factor;
        assert_eq!(largest_prime_factor(13_195), 29);
    }
}
