use std::io;

fn smallest_number(n: i32) -> i32 {
    let b = (n as f64).log2().floor() as i32 + 1;
    (1 << b) - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid input");
    let result = smallest_number(n);
    println!("{}", result);
}