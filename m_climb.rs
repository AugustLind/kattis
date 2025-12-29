use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let m: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");

    println!("{}", n * m);
}
