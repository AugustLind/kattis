use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let a: i32 = input.trim().parse().expect("Failed");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let b: i32 = input.trim().parse().expect("Failed");

    if b > a {
        println!("FAKE NEWS!");
    } else if a == b {
        println!("WORLD WAR 3!");
    } else{
        println!("MAGA!");
    }   
}