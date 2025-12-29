use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i64 = input.trim().parse().expect("Failed");

    let sum: f64 = std::f64::consts::PI * n as f64;

    println!("{}", sum);

}
