use std::io;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits
    let b = (n as f64).log2() as i32 + 1;
    
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Input the number
    io::stdin().read_line(&mut input)?;
    let n: i32 = input.trim().parse().expect("Invalid input: expected an integer");
    
    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}