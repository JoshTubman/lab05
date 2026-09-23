mod Digit_Sum;
mod Divisor_Sum;
mod Is_Prime;
mod Is_Even;
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
fn is_prime(n: i32){ 
let mut divisor_count: i32 =0;
let mut divider : i32 = n;
let mut is_prime: bool;
while divider >0{
if n%divider==0{
divisor_count +=1;
}else {}
divider -=1;
}if divisor_count == 2{
is_prime== true
}else{
is_prime== false
}}
fn count_divisors (n: i32){
let mut divisor_count: i32 =0;
let mut divider: i32 = n
while divider >0{
if n%divider==0{
divisor_count+=1;
}else {}
divider-=1;
}}
divisor_count==divisor_count
}
