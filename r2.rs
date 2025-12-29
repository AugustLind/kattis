use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
        
    let r1 = nums[0];
    let s = nums[1];
    let r2 = 2*s -r1;
    println!("{}", r2);
}
