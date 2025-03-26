use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input");
    let k: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Initialize grid and limits
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        let row: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    let mut limits = vec![0; n];
    for i in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        limits[i] = buffer.trim().parse().expect("Invalid input");
    }

    // Calculate the result
    let result = max_sum(&grid, &limits, k);

    // Output the result
    println!("{}", result);
}

// Function to compare two integers in descending order
fn cmp_desc(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
}

// Function to calculate the maximum sum
fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<usize>, k: usize) -> i64 {
    let len: usize = limits.iter().sum();
    let mut lst = Vec::with_capacity(len);

    for (i, limit) in limits.iter().enumerate() {
        let mut sorted_row = grid[i].clone();
        sorted_row.sort_by(cmp_desc);
        lst.extend_from_slice(&sorted_row[0..*limit]);
    }

    lst.sort_by(cmp_desc);
    lst[0..k].iter().map(|&x| x as i64).sum()
}