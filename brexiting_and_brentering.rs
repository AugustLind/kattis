use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input = input.trim(); 
    
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    for i in (0..input.len()).rev() {
        let ch = input.chars().nth(i).unwrap();
        
        if vowels.contains(&ch) {
            println!("{}ntry", &input[0..i+1]);
            return;
        }
    }
}