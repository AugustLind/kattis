use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed");
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let calories: i32 = input.trim().parse().expect("Failed");
        let mut number_of_bottles = calories / 400;
        if number_of_bottles * 400 < calories{
            number_of_bottles += 1;
        }
        println!("{}", number_of_bottles);
    }
}