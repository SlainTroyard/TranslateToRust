use std::cmp;
use std::error::Error;
use std::io::{self, Read, Write};

fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    // Sort the array in ascending order.
    arr.sort_unstable();

    let mut prev = std::i32::MIN;
    let mut distinct_count = 0;

    // Iterate over each element in the sorted array
    for &value in arr.iter() {
        // Determine the candidate value x as the maximum of (prev+1) and (value - diff)
        let x = cmp::max(prev + 1, value - diff);
        // Check if x is within the allowed range [value - diff, value + diff]
        if x <= value + diff {
            distinct_count += 1;
            prev = x; // Update the previous value with x
        }
    }
    distinct_count
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire standard input into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and collect tokens.
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // There should be at least two tokens: one for the array size and one for diff.
    if tokens.len() < 2 {
        writeln!(io::stderr(), "Insufficient input.")?;
        std::process::exit(1);
    }

    // Parse the array size and diff.
    let n: usize = tokens[0].parse().map_err(|_| "Invalid integer for array size")?;
    let diff: i32 = tokens[1].parse().map_err(|_| "Invalid integer for diff")?;

    // Ensure there are enough tokens for the array elements.
    if tokens.len() < 2 + n {
        writeln!(io::stderr(), "Insufficient array elements provided.")?;
        std::process::exit(1);
    }

    // Parse the next n tokens as the array elements.
    let mut arr: Vec<i32> = tokens[2..2 + n]
        .iter()
        .map(|s| s.parse().map_err(|_| "Invalid integer in array"))
        .collect::<Result<_, _>>()?;

    // Compute the result using the algorithm.
    let result = max_distinct_elements(&mut arr, diff);

    // Output the result followed by a newline.
    println!("{}", result);

    Ok(())
}