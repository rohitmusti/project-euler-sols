/*
 * Prompt:
 * 2520 is the smallest number that can be divided by each of the numbers from 1 
 * to 10 without any remainder.  
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * My theory is that it is a product of the prime factors
 */

mod test;

fn main() {
    let target: u32 =20;
    let result = smallest_divis_by(target);
    println!("The smallest number divisible by all the numbers between 1 and {} is: {}", target, result);
}

fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }
    let sqrt: u32 = (n as f64).sqrt() as u32;
    let mut flag = true;

    for test in 2..=sqrt {
        if (n as f64) % (test as f64) == 0.0 {
            flag = false;
            break;
        }
    }
    flag
}

fn prime_factor_count(factor: &u32, n: u32) -> u32 {
    let mut result = *factor;
    while result <= n {
        result *= *factor
    }
    result / *factor
}

fn smallest_divis_by(n: u32) -> u32 {
    let mut all_primes: Vec<u32> = Vec::new();
    for cand in 2..=n {
        if is_prime(cand){
            all_primes.push(cand);
        }
    }
    let final_primes: Vec<_> = all_primes.iter().map(|x| prime_factor_count(x, n)).collect();
    final_primes.iter().product()

}
