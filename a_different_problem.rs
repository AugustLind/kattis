use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        
        println!("{}", (a - b).abs());
    }
}