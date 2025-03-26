// Problem: Weekly Contest 430 Problem 1
use std::io::{self, BufRead};

fn minimum_operations(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: &Vec<usize>) -> i32 {
    // Create a mutable copy of the grid
    let mut cal_grid = grid.clone();

    // Early return for single row grid
    if grid_size == 1 {
        return 0;
    }

    let mut ans = 0;
    
    // For each column
    for i in 0..grid_col_size[0] {
        // For each row (starting from the second row)
        for j in 1..grid_size {
            // If current value is less than or equal to the value above it
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // Calculate how much to increment the current value
                let increment = cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                ans += increment;
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid dimensions
    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let grid_size = dimensions[0];
    let grid_col_size = dimensions[1];
    
    // Initialize grid and column sizes
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    let col_sizes = vec![grid_col_size; grid_size];
    
    // Read grid values
    for i in 0..grid_size {
        let row: Vec<i32> = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..grid_col_size {
            grid[i][j] = row[j];
        }
    }
    
    // Calculate and print the result
    let result = minimum_operations(&grid, grid_size, &col_sizes);
    println!("{}", result);
    
    Ok(())
}