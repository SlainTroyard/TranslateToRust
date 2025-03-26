use std::io::{self, BufRead};
use std::cmp::Reverse;

fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<usize>, k: usize) -> i64 {
    // Flatten the relevant elements from each row of the grid based on limits.
    let mut lst = Vec::new();
    for (row, &limit) in grid.iter().zip(&limits) {
        let mut sorted_row = row.clone();
        sorted_row.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
        lst.extend(&sorted_row[..limit.min(sorted_row.len())]); // Take up to `limit` elements
    }

    // Sort the combined list in descending order and sum the top k elements.
    lst.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    lst.iter().take(k).map(|&x| x as i64).sum()
}

fn main() {
    // Read from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for n (rows), m (columns), and k
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let n: usize = first_line_split.next().unwrap().parse().unwrap();
    let m: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: usize = first_line_split.next().unwrap().parse().unwrap();

    // Parse the grid rows
    let mut grid = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        grid.push(row);
    }

    // Parse the limits array
    let limits_line = lines.next().unwrap().unwrap();
    let limits: Vec<usize> = limits_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Compute the result using the `max_sum` function
    let result = max_sum(grid, limits, k);

    // Print the result as required
    println!("{}", result);
}