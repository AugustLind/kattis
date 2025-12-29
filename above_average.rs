use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let all_nums: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        // Skip the first number (count) and take only the grades
        let nums: Vec<i32> = all_nums[1..].to_vec();
        
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }
        
        let average = sum / (nums.len() as i32);
        let mut count = 0;
        
        for i in 0..nums.len() {
            if nums[i] > average {
                count += 1;
            }
        }
        
        let percentage: f32 = (count as f32) / (nums.len() as f32) * 100.0;
        println!("{:.3}%", percentage);
    }
}