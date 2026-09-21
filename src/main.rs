use count_divisors;
use is_prime;
fn main() {
    let start = read_i32("Start of range:");
    let end = read_i32("End of range:");
    for n in start..=end {
        println!(
            "{n}: even={}, digit_sum={}, prime={}, divisors={}",
            is_even(n),
            digit_sum(n),
            is_prime(n),
            count_divisors(n)
        );
    }
}
