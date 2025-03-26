use std::io::{self, Read};
use std::cmp::{max, min};

fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort();

    for &val in arr.iter() {
        // Choose max of (prev + 1) and (val - diff) to ensure distinct elements
        let x = max(prev + 1, val - diff);

        // Check if the chosen value is within allowed range
        if x <= val + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    // Read all input into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut values = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid input"));
    
    // Parse input in the same order as the original C code
    let n = values.next().unwrap() as usize;
    let diff = values.next().unwrap();
    
    // Collect array elements
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(values.next().unwrap());
    }
    
    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);
    
    // Output the result
    println!("{}", result);
}