use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    nums.sort();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let order = input.trim();
    
    let result: Vec<i32> = order
        .chars()
        .map(|ch| match ch {
            'A' => nums[0],
            'B' => nums[1],
            'C' => nums[2],
            _ => 0,
        })
        .collect();
    
    println!("{} {} {}", result[0], result[1], result[2]);
}