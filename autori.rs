use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let parts: Vec<&str> = input.trim().split('-').collect();
    
    for part in &parts {
        if let Some(first_char) = part.chars().next() {
            print!("{}", first_char);
        }
    }
}