/*
 * Prompt:
 * A palindromic number reads the same both ways. The largest palindrome made
 * from the product of two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * Two possible methods:
 *
 * start from the number
 * we know that the largest possible 3 digit product is 999 * 999
 * so do we just iterate downwards until we find a palindrome and
 * then check for that palindrome's 3 digit factors until we find one w/ two
 * 3-digit factors?
 *
 * start from the factors, try factors until you find the biggest palindrome
 */

mod test;

fn main() {
    println!("Hello, world!");
}

fn is_palindrome(num: u32) -> bool {
    let num_str = num.to_string();
    let mid_point = &num_str.len() / 2;
    let half1;
    let half2;
    if &num_str.len() % 2 == 0 {
        half1 = &num_str[..mid_point];
        half2 = &num_str[mid_point..];
    } else {
        half1 = &num_str[..mid_point];
        half2 = &num_str[(mid_point + 1)..];
    }
    half1 == half2.chars().rev().collect::<String>()
}

