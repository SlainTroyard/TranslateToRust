```rust
use std::error::Error;
use std::io::{self, BufRead};

/// Reads all integer tokens from stdin and stores them in a vector.
fn read_all_int_tokens() -> Result<Vec<i32>, Box<dyn Error>> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        for part in line.split_whitespace() {
            tokens.push(part.parse()?);
        }
    }
    Ok(tokens)
}

/// Translated version of the C function maxSum.
/// Sorts each row in descending order, takes the top 'limits[i]' elements from each row,
/// collects them into a single list, sorts that list in descending order, and sums the top 'k' elements.
fn max_sum(grid: &mut [Vec<i32>], limits: &[i32], k: usize) -> i64 {
    // Calculate total number of elements we'll collect based on limits.
    let len: usize = limits.iter().map(|&x| x as usize).sum();
    let mut lst = Vec::with_capacity(len);

    // Sort each row descending and collect the top limits[i] elements.
    for (i, row) in grid.iter_mut().enumerate() {
        row.sort_unstable_by(|a, b| b.cmp(a));
        for &val in row.iter().take(limits[i] as usize) {
            lst.push(val);
        }
    }

    // Sort the collected elements descending.
    lst.sort_unstable_by(|a, b| b.cmp(a));

    // Sum the top k elements into a 64-bit integer.
    lst.iter().take(k).map(|&v| v as i64).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all tokens from stdin.
    let tokens = read_all_int_tokens()?;
    let mut index = 0;

    // Parse n, m, k
    let n = tokens[index] as usize; index += 1;
    let m = tokens[index] as usize; index += 1;
    let k = tokens[index] as usize; index += 1;

    // Read the grid (n rows, each with m integers).
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(m);
        for _ in 0..m {
            row.push(tokens[index]);
            index += 1;
        }
        grid.push(row);
    }

    // Read the limits array (n integers).
    let mut limits = Vec::with_capacity(n);
    for _ in 0..n {
        limits.push(tokens[index]);
        index += 1;
    }

    // Compute the result using the translated max_sum function.
    let result = max_sum(&mut grid, &limits, k);

    // Print the result in the same format as the original C code.
    println!("{}", result