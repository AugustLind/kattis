use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i64 = input.trim().parse().expect("Failed");
    println!("{}", n - 1);
}