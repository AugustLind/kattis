use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n : i32 = input.trim().parse().expect("Failed");

    for i in 1..n+1{
        println!("{} Abracadabra", i);
    }   
    
}
