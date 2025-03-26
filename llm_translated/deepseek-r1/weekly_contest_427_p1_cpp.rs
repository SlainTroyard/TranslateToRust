use std::io::{self, Read};

fn main() {
    // Read all input as a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split into whitespace-separated tokens and parse as i32
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    let mut tokens_iter = tokens.into_iter();

    // Read array size (first token)
    let n_input = tokens_iter.next().expect("Missing array size");
    if n_input < 0 {
        panic!("Array size cannot be negative");
    }
    let n = n_input as usize;

    // Read array elements (next n tokens)
    let a: Vec<i32> = tokens_iter.by_ref().take(n).collect();
    if a.len() != n {
        panic!("Not enough elements in the array");
    }

    // Generate transformed array
    let transformed = construct_transformed_array(&a);

    // Print space-separated elements with trailing newline
    for num in transformed {
        print!("{} ", num);
    }
    println!();
}

/// Constructs transformed array using the same logic as original C++ code
fn construct_transformed_array(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
    let n_i32 = n as i32;

    for (i, res_item) in res.iter_mut().enumerate().take(n) {
        let i_i32 = i as i32;
        let offset = a[i] % n_i32;
        // Calculate index with proper handling of negative values
        let idx = ((i_i32 + offset + n_i32) % n_i32) as usize;
        *res_item = a[idx];
    }

    res
}