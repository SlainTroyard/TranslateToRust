// Weekly Contest 436 Problem 1 in Rust
// Translated from C to Rust, preserving the same algorithm and I/O format.

use std::io::{self, BufRead};

/// Sorts the matrix according to the same logic as the original C code.
/// - For diagonals starting at (i,0) for i from 0..n, sort in descending order.
/// - For diagonals starting at (0,i) for i from 1..n, sort in ascending order.
///
/// This function returns a newly sorted matrix (like the C version returns a pointer).
fn sortMatrix(mut grid: Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    // First set: sort diagonals (i,0) in descending order
    for i in 0..n {
        let mut diag = Vec::new();
        let (mut r, mut c) = (i, 0);
        // Collect diagonal
        while r < n && c < n {
            diag.push(grid[r][c]);
            r += 1;
            c += 1;
        }
        // Sort descending
        diag.sort_by(|a, b| b.cmp(a));
        // Write back
        let (mut r, mut c) = (i, 0);
        let mut index = 0;
        while r < n && c < n {
            grid[r][c] = diag[index];
            index += 1;
            r += 1;
            c += 1;
        }
    }

    // Second set: sort diagonals (0,i) in ascending order
    for i in 1..n {
        let mut diag = Vec::new();
        let (mut r, mut c) = (0, i);
        // Collect diagonal
        while r < n && c < n {
            diag.push(grid[r][c]);
            r += 1;
            c += 1;
        }
        // Sort ascending
        diag.sort();
        // Write back
        let (mut r, mut c) = (0, i);
        let mut index = 0;
        while r < n && c < n {
            grid[r][c] = diag[index];
            index += 1;
            r += 1;
            c += 1;
        }
    }

    grid
}

fn main() {
    // Read input: first the integer n, then n*n integers for the matrix.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n.
    let n_line = lines.next().unwrap().unwrap();
    let n: usize = n_line.trim().parse().expect("Failed to parse n as usize");

    // Read the remaining integers in the input for the matrix.
    let mut all_numbers = Vec::with_capacity(n * n);
    for line_result in lines {
        let line = line_result.unwrap();
        for token in line.split_whitespace() {
            all_numbers.push(token.parse::<i32>().expect("Failed to parse integer"));
            if all_numbers.len() == n * n {
                break;
            }
        }
        if all_numbers.len() == n * n {
            break;
        }
    }

    // Build the grid (matrix).
    let mut grid = vec![vec![0; n]; n];
    let mut index = 0;
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = all_numbers[index];
            index += 1;
        }
    }

    // Sort the matrix using the same logic as the original C program.
    let result = sortMatrix(grid, n);

    // Print the result with the exact same format (space after each number, then newline).
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}