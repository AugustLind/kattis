use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed");
    let n: f64 = input.trim().parse().expect("Failed");
    
    println!("{:.4}", n - 0.3);
}