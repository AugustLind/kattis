use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
        
    let x = nums[0];
    let y = nums[1];
    let n = nums[2];

    for i in 1..(n+1) {
        if i % x == 0 && i % y == 0{
            println!("FizzBuzz");
        } else if i % x == 0 {
            println!("Fizz");
        } else if i % y == 0 {
            println!("Buzz");
        } else{
            println!("{}", i);
        }
    }
}