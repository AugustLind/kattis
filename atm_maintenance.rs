use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
        
    let n = nums[0] as usize;
    let mut bank = nums[1];
    let mut result = String::new();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let people: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed"))
        .collect();
        

    for i in 0..n{
        if (bank - people[i]) >= 0 {
            bank -= people[i];
            result += "1";
        } else {
            result += "0";
        }
    }
    println!("{}", result);
}

