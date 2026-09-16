fn main() {
    let price: f64 = 2.5;
    let quantity: f64 = 4.0;
    let total: f64;
    total = (price * quantity);

    println!("Total: {total}");








\\ functions 3 and 4
    let number: i32=6;
let mut count_divisor: i32=0;
let mut dividend: i32=number;
let mut number_divided: i32;

while dividend >=1 {
number_divided=number%dividend;
if number_divided == 0{
count_divisor+=1;
}else{
}dividend-=1;
}
println!("number has:{}", {count_divisor}, " divisors");

let isprime: bool;
if count_divisor==2{
isprime=true;
}else{
isprime=false;
}if isprime==true{
println!("{} is prime", number);
}else{
println!("{} is not prime", number);
}

}


