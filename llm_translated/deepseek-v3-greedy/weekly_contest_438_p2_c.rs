use std::cmp::Ordering;
use std::io::{self, BufRead};

// Custom comparator function to sort in descending order
fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

// Function to calculate the maximum sum of the top k elements
fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<usize>, k: usize) -> i64 {
    // Flatten the grid by taking the top `limits[i]` elements from each row
    let mut lst: Vec<i32> = grid
        .into_iter()
        .zip(limits.into_iter())
        .flat_map(|(mut row, limit)| {
            row.sort_by(cmp);
            row.into_iter().take(limit)
        })
        .collect();

    // Sort the flattened list in descending order
    lst.sort_by(cmp);

    // Sum the top k elements
    lst.into_iter().take(k).map(|x| x as i64).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, m, k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Read the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    // Read the limits
    let line = lines.next().unwrap().unwrap();
    let limits: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print the result
    let result = max_sum(grid, limits, k);
    println!("{}", result);
}