use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i64 = input.trim().parse().expect("Failed");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
    
    let mut sum: i64 = 0;
    for num in nums{
        sum += num;
    }

    
    println!("{}", sum/n);
}