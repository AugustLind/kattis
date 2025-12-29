use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: i32 = input.trim().parse().expect("Failed");

    let upper: i32 = (n / 100) * 100 + 99;
    let under: i32 = ((n/100) -1) * 100 + 99;

    if under <= 0{
        println!("{}", upper);
    }
    else {
        let dist_upper = upper - n;
        let dist_under = n - under;
        if dist_under < dist_upper{
            println!("{}", under);
        } else{
            println!("{}", upper);
        }

    }
}
