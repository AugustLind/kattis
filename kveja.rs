use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let m = input.trim().to_string();
    
    println!("Kvedja,");
    println!("{}", m);
}
