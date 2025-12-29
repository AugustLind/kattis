use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed");
    
    let mut vec = Vec::new();
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let new_input: i32 = input.trim().parse().expect("Failed");
        vec.push(new_input); 
    }
    
    vec.sort();
    let largest_number = vec[(n - 1) as usize];  
    let mut printed: bool = false;
    let mut pos = 0;
    for i in 1..largest_number {
        if pos < vec.len() && vec[pos] == i {  
            pos += 1; 
        } else {
            println!("{}", i);
            printed = true;
        }
    }
    if printed == false{
        println!("good job");
    }
    
}