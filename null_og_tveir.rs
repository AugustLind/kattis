use std::io;
use std::collections::VecDeque;

fn count_valid(n: i64) -> i64 {
    let mut count = 0;
    let mut queue = VecDeque::new();
    
    queue.push_back(0);
    queue.push_back(2);
    
    while let Some(num) = queue.pop_front() {
        if num > n {
            continue;
        }
        
        count += 1;
        
        if num == 0 {
            continue;
        }
        
        if num > n / 10 {
            continue;
        }
        
        let next0 = num * 10;
        let next2 = num * 10 + 2;
        
        if next0 <= n {
            queue.push_back(next0);
        }
        if next2 <= n {
            queue.push_back(next2);
        }
    }
    
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    
    println!("{}", count_valid(n));
}