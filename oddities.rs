use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let a: i32 = input.trim().parse().expect("Failed");
        if a % 2 == 0{
            println!("{} is even", a);
        }
        else{
            println!("{} is odd", a);
        }
    }
}