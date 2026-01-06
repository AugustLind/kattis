use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let a: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let b: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let c: i32 = input.trim().parse().expect("Failed");

    println!("{}", c - b - a);
}