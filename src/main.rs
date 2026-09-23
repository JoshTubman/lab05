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
// is prime function
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
is_prime: true
}else{
is_prime: false
}}
// divisor count function
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
// is even function
fn is_even (n: i32){
let mut even: bool;
while n>0{
n-=2;
}if n == 0{
even: true
}else{
even: false
}}
//Digit sum function 
fn digit_sum (n: i32){
let mut digit: i32;
let mut sum_digit: i32 =0;
let mut n_div10: i32 = n;
while n>0{
digit==n_div10%10;
sum_digit+=digit;
n_div10/=10;
}
return:sum_digit
}
