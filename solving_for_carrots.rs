use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut iter = input.split_whitespace();
    let _n: i32 = iter.next().unwrap().parse().unwrap(); 
    let p: i32 = iter.next().unwrap().parse().unwrap();  
    
    println!("{}", p);
}