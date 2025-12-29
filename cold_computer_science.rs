use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let _n: i32 = input.trim().parse().expect("Failed");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let cold = input
        .split_whitespace()
        .filter(|x| x.parse::<i32>().unwrap() < 0)
        .count();
    
    println!("{}", cold);
}