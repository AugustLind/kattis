use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let lengths: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let row1: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let row2: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    for i in 0..lengths[0]{
        if row2.contains(&row1[i as usize]){
            print!("{} ", row1[i as usize]);
        }
    }

}
