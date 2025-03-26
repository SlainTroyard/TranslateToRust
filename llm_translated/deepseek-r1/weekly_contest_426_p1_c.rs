use std::io;

// Calculate the smallest number with the same number of bits as `n`
fn smallest_number(n: i32) -> i32 {
    // Convert to u32 to use ilog2, which gives the floor of log2(n)
    let bits = (n as u32).ilog2() + 1;
    // Compute 2^bits - 1 using bit shifting on u32 to avoid overflow issues
    ((1u32 << bits) - 1) as i32
}

fn main() {
    let mut input = String::new();
    // Read input line and handle potential errors
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Parse input as i32 with error handling
    let n: i32 = input.trim().parse().expect("Invalid integer input");
    
    // Compute and print the result
    let result = smallest_number(n);
    println!("{}", result);
}