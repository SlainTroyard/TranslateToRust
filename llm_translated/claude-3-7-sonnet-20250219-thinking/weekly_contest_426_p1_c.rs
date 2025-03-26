use std::io;

/// Returns the smallest number with the same number of bits as `n`
fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits
    // In Rust, we can use the built-in leading_zeros function instead of log2
    let b = 32 - n.leading_zeros() as i32;
    
    // Return 2^b - 1, which gives a number with b consecutive '1' bits
    (1 << b) - 1
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Input the number
    io::stdin().read_line(&mut input)?;
    let n: i32 = input.trim().parse().expect("Invalid input");
    
    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}