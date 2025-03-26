// Weekly Contest 430 Problem 1 in Rust
// Translated from the provided C code, preserving the same logic and I/O format.
//
// The program reads two integers (gridSize and gridColSize) from stdin,
// then reads gridSize rows each containing gridColSize integers, which together form the grid.
//
// It then computes the minimum number of increments needed so that, for each column,
// every element is strictly greater than the element above it.
//
// Finally, it prints the result to stdout.

use std::io::{self, BufRead};

/// Translated version of the `minimumOperations` function from the C code.
fn minimum_operations(grid: &[Vec<i32>], col_sizes: &[usize]) -> i32 {
    let grid_size = grid.len();

    // Create a copy of the input grid (equivalent to malloc + copying in C).
    let mut cal_grid = grid.to_vec();

    // If there's only one row, no operations are needed.
    if grid_size == 1 {
        return 0;
    }

    let mut ans = 0;

    // For each column, ensure each subsequent row is strictly greater
    // than the previous row in that column.
    for i in 0..col_sizes[0] {
        for j in 1..grid_size {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get gridSize and gridColSize
    let first_line = lines.next().expect("Expected at least one line of input");
    let mut tokens = first_line
        .split_whitespace()
        .map(|t| t.parse::<usize>().expect("Invalid integer"));

    // gridSize and gridColSize
    let grid_size = tokens.next().expect("Missing gridSize");
    let grid_col_size = tokens.next().expect("Missing gridColSize");

    // Prepare to read the grid
    let mut grid = Vec::with_capacity(grid_size);
    let mut col_sizes = Vec::with_capacity(grid_size);

    // Read each row of the grid
    for _ in 0..grid_size {
        let line = lines.next().expect("Expected more lines for grid rows");
        let row_values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid integer in grid"))
            .collect();

        // In the original C code, gridColSize is fixed for each row
        if row_values.len() != grid_col_size {
            panic!("Each row must contain exactly {} integers", grid_col_size);
        }

        // Store the row and record its column size
        grid.push(row_values);
        col_sizes.push(grid_col_size);
    }

    // Compute the result
    let result = minimum_operations(&grid, &col_sizes);

    // Print the result
    println!("{}", result);
}