use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let mut n: i32 = input.trim().parse().expect("Failed");

    loop{
        n += 1;
        let square = n * n;
        if !square.to_string().contains("6"){
            println!("{}", n);
            break;
        }
    }

}
