use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let x : i32 = input.trim().parse().expect("Failed");
    input.clear();
     io::stdin().read_line(&mut input).expect("Failed");
    let y : i32 = input.trim().parse().expect("Failed");

    if x > 0 && y > 0{
        println!("{}", 1);
    } else if x > 0 {
        println!("{}", 4);
    } else if y < 0{
        println!("{}", 3);
    } else{
        println!("{}", 2);
    }
}