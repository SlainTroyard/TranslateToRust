use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    // Split the input on whitespace into an iterator of tokens
    let mut tokens = input.split_whitespace();
    
    // Parse n, m, k from the first three tokens.
    // n: number of rows, m: number of columns in each row, k: number of elements to sum.
    let n: usize = tokens
        .next()
        .ok_or("Missing n")?
        .parse()?;
    let m: usize = tokens
        .next()
        .ok_or("Missing m")?
        .parse()?;
    let k: usize = tokens
        .next()
        .ok_or("Missing k")?
        .parse()?;
    
    // Build the grid. There are n rows and each row has m integers.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row: Vec<i32> = Vec::with_capacity(m);
        for _ in 0..m {
            let num: i32 = tokens
                .next()
                .ok_or("Missing grid element")?
                .parse()?;
            row.push(num);
        }
        grid.push(row);
    }
    
    // Read limits for each row. There will be n limit values.
    let mut limits: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let lim: usize = tokens
            .next()
            .ok_or("Missing limit element")?
            .parse()?;
        limits.push(lim);
    }
    
    // Compute the result using the max_sum function.
    let result = max_sum(&mut grid, &limits, k);
    
    // Print the result followed by a newline.
    println!("{}", result);
    
    Ok(())
}

/// Computes the maximum sum by choosing elements according to the problem description.
/// For each row, the row is sorted in descending order and the first `limit[i]` elements
/// are collected into a list. That list is then sorted in descending order and the sum of
/// the first `k` elements is returned.
fn max_sum(grid: &mut [Vec<i32>], limits: &[usize], k: usize) -> i64 {
    // Determine the total number of elements to allocate in the list.
    let total_count: usize = limits.iter().sum();
    let mut lst: Vec<i32> = Vec::with_capacity(total_count);
    
    // Process each row: sort in descending order and pick the first `limits[i]` elements.
    for (i, row) in grid.iter_mut().enumerate() {
        // Sort each row in descending order.
        row.sort_unstable_by(|a, b| b.cmp(a));
        // Push the first `limits[i]` elements into lst.
        for j in 0..limits[i].min(row.len()) {
            lst.push(row[j]);
        }
    }
    
    // Sort the aggregated list in descending order.
    lst.sort_unstable_by(|a, b| b.cmp(a));
    
    // Sum the first `k` elements in the sorted list, casting to i64.
    lst.iter().take(k).map(|&x| x as i64).sum()
}