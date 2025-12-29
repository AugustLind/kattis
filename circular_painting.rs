use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    let mut sum: f32 = 0.0;
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let nums: Vec<f32> = input
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
                
        sum += (nums[0] / 360.0) * std::f32::consts::PI * (nums[2]*nums[2] -nums[1]*nums[1]);
    }
    println!("{}", sum);

}
