use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    let mut sum: i32 = 0;
    for i in 0..n{
        sum += nums[i as usize];
    }

    println!("{}", sum);
}
