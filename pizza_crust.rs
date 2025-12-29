use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let r = nums[0] as f64;
    let c = nums[1] as f64;
    
    let cheese_radius = r - c;
    
    let percentage = if cheese_radius <= 0.0 {
        0.0
    } else {
        (cheese_radius * cheese_radius) / (r * r) * 100.0
    };
    
    println!("{}", percentage);
}