use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace to get individual tokens
    let mut tokens = input.split_whitespace();

    // Read the size of the array
    let n: usize = tokens
        .next()
        .ok_or("Expected the size of the array")?
        .parse()?;

    // Read the elements of the array
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = tokens
            .next()
            .ok_or("Expected an integer element for the array")?
            .parse()?;
        a.push(num);
    }

    // Construct the transformed array.
    // For every index i, we use the formula:
    //   result[i] = a[(i + a[i] % n + n) % n]
    // The extra "+ n" ensures the index is non-negative.
    let mut res: Vec<i32> = vec![0; n];
    for i in 0..n {
        // Convert i to i32 to do arithmetic with a[i]
        let i_as_i32 = i as i32;
        // Compute (a[i] % n); note that a[i] may be negative so we adjust by adding n and taking modulo n again.
        let index = ((i_as_i32 + (a[i] % n as i32) + n as i32) % n as i32) as usize;
        res[i] = a[index];
    }

    // Print the transformed array with space separated values and a newline at the end
    for num in &res {
        print!("{} ", num);
    }
    println!();

    Ok(())
}