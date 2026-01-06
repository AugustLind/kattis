use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    let king = 1 - nums[0];
    let queen = 1 - nums[1];
    let rooks = 2 - nums[2];
    let bishop = 2 - nums[3];
    let knights = 2 - nums[4];
    let pawns = 8 - nums[5];
    
    println!("{} {} {} {} {} {}", king, queen, rooks, bishop, knights, pawns);
}
