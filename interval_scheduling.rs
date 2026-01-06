use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse n");
    
    let mut intervals: Vec<(i32, i32)> = Vec::new();
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let parts: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse"))
            .collect();
        
        let start = parts[0];
        let end = parts[1];
        intervals.push((start, end));
    }
    
    intervals.sort_by_key(|interval| interval.1);    
    
    let mut sum = 1;
    let mut stop_time = intervals[0].1;  
    for interval in &intervals[1..] {  
        if interval.0 >= stop_time {  
            stop_time = interval.1;
            sum += 1; 
        }
    }
    
    println!("{}", sum);
}