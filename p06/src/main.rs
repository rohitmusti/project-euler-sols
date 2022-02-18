/* Prompt:
 * The sum of the squares of the first ten natural numbers is 385.  The square of 
 * the sum of the first ten natural numbers is 3025.  Hence the difference between 
 * the sum of the squares of the first ten natural numbers and the square of the 
 * sum is 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */
mod test;

fn main() {
    let target: u32 = 100;
    let sol = diff_square_of_sum_of(target);
    println!("The difference between the squares of sum and the sum of squares for all positive numbers up to {}: {}", target, sol);
}

fn sum_of_squares(n: u32) -> u32 {
    let squares_vector: Vec<u32> = (1..=n).map(|x| x.pow(2)).collect::<Vec<u32>>();
    let sum: u32 = squares_vector.iter().sum();
    sum
}

fn squares_of_sum(n: u32) -> u32 {
    let normal_vector: Vec<u32> = (1..=n).collect();
    let normal_vector_sum: u32 = normal_vector.iter().sum();
    let sum_squared: u32 = normal_vector_sum.pow(2);
    sum_squared
}

fn diff_square_of_sum_of(n:u32) -> u32 {
    squares_of_sum(n) - sum_of_squares(n)
}
