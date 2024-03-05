use superman_chicken_rescue::solve;

fn main() {
    use std::io::{stdin, stdout, Write};

    print!("input (chicken_amount roof_size): ");
    let _ = stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut nums = buffer.trim().split(' ');
    let amount: usize = nums
        .next()
        .unwrap()
        .parse()
        .expect("Amount not a positive integer");
    let size: u32 = nums
        .next()
        .unwrap()
        .parse()
        .expect("Size not a positive integer");

    print!("chicken positions: ");
    let _ = stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Position not a positive integer"))
        .collect();

    println!("result: {}", solve(amount, size, &numbers));
}
