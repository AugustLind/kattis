use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let n: usize = input.trim().parse().expect("Failed");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let outcomes: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let mut freq = HashMap::new();
    for &outcome in &outcomes {
        *freq.entry(outcome).or_insert(0) += 1;
    }
    
    let mut max_unique = -1;
    let mut winner_index = -1;
    
    for (i, &outcome) in outcomes.iter().enumerate() {
        if freq[&outcome] == 1 && outcome > max_unique {
            max_unique = outcome;
            winner_index = (i + 1) as i32;
        }
    }
    
    if winner_index == -1 {
        println!("none");
    } else {
        println!("{}", winner_index);
    }
}