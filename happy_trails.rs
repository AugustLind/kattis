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

        let angle_radians = nums[0].to_radians();
        let result = angle_radians.sin();
        sum += nums[1] * result;
    }
    println!("{:.2}", sum);

}
