use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i64 = input.trim().parse().expect("Failed");
    
    let modulo: i128 = 1_000_000_007;
    let mut mult: i128 = 1;
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let a: i128 = input.trim().parse().expect("Failed");
        mult = (mult * a) % modulo;
    }
    
    println!("{}", mult);
}