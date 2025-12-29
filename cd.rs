use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    loop {
        let first_line = lines.next().unwrap();
        let nums: Vec<usize> = first_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let (n, m) = (nums[0], nums[1]);
        
        if n == 0 && m == 0 {
            break;
        }
        
        let mut jack_set: HashSet<u32> = HashSet::new();
        for _ in 0..n {
            jack_set.insert(lines.next().unwrap().trim().parse().unwrap());
        }
        
        let mut count = 0;
        for _ in 0..m {
            let cd: u32 = lines.next().unwrap().trim().parse().unwrap();
            if jack_set.contains(&cd) {
                count += 1;
            }
        }
        
        println!("{}", count);
    }   
}