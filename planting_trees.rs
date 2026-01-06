use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");
    
      
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
           
    let mut trees: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();


    trees.sort();
    trees.reverse();
    let mut days = 0;

    for i in 0..n{
        days = cmp::max(days, i + trees[i as usize]);
    }
    println!("{}", days + 2);
}