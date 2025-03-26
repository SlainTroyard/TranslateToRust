use std::io::{self, BufRead};

// Calculate the smallest number with the same number of bits as `n`
fn smallest_number(n: i32) -> i32 {
    let b = (n as f64).log2().floor() as i32 + 1; // Calculate the number of bits
    (1 << b) - 1                                  // Return 2^b - 1
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the number
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    println!("{}", result);

    Ok(())
}