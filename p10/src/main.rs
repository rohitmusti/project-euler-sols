/* Prompt:
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.  Find the sum of all the
 * primes below two million.
 */

mod test;

fn main() {
    let target = 2000000;
    let sol: u64 = all_primes(target).iter().sum();
    println!("The sum of all primes beneath {}: {}", target, sol);
}

fn all_primes(n: u64) -> Vec<u64> {
    let mut sol: Vec<u64> = Vec::new();
    for i in 2..n {
        if is_prime(i) {
            sol.push(i);
        }
    }
    sol
}

fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }
    let sqrt: u64 = (n as f64).sqrt() as u64;
    let mut flag = true;

    for test in 2..=sqrt {
        if (n as f64) % (test as f64) == 0.0 {
            flag = false;
            break;
        }
    }
    flag
}
