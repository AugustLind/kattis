use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let p: i32 = input.trim().parse().expect("Failed");

    println!("{}", (n-1) * p);
}
