use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    let side: f64 = (n as f64).sqrt();  
    let result: f64 = side * 4.0;
    println!("{}", result);
}