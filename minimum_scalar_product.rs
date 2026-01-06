use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i64 = input.trim().parse().expect("Failed");
    
    for i in 1..(n+1) {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let vector_len: i64 = input.trim().parse().expect("Failed");
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
           
        let mut vector_1: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
           
        let mut vector_2: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        vector_1.sort_by(|a, b| b.partial_cmp(a).expect("Failed"));
        vector_2.sort_by(|a, b| a.partial_cmp(b).expect("Failed"));

        let mut sum: i64 = 0;
        for j in 0..vector_len{
            sum += vector_1[j as usize] * vector_2[j as usize];
        }
        println!{"Case #{}: {}", i , sum};
    }
}