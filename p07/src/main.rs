/* Prompt:
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
 * that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 *
 * Approach:
 * There isn't a known prime number algorithm, so I think the best approach is to brute force
 * iterate starting at 3 and only consider odd numbers (2 is the only even prime)
 */

mod test;

fn main() {
    let target: u32 = 10_001;
    let sol = find_nth_prime(target);
    print!("The {} prime number: {}", target, sol);
}

fn find_nth_prime(n: u32) -> u32 {
    let mut counter: u32 = 1;
    let mut result: u32 = 0;

    for i in (3..).step_by(2) {
        if is_prime(&i) {
            counter += 1;
            result = i;
        }

        if n == counter {
            break;
        }
    }

    result
}

fn is_prime(n: &u32) -> bool {
    if *n == 1 {
        return false;
    } else if *n == 2 {
        return true;
    }
    let sqrt: u32 = (*n as f64).sqrt() as u32;
    let mut flag = true;

    for test in 2..=sqrt {
        if (*n as f64) % (test as f64) == 0.0 {
            flag = false;
            break;
        }
    }
    flag
}
