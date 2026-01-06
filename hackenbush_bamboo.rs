use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed");
    let _n: usize = input.trim().parse().expect("Failed");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let heights: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
    
    let mut xor_sum = 0;
    for height in heights {
        xor_sum ^= height;
    }
    
    println!("*{}", xor_sum);
}