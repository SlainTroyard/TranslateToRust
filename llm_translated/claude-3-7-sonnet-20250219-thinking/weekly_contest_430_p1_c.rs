use std::io::{self, BufRead};

/// Calculate the minimum number of operations to make sure values strictly increase from top to bottom in each column
fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    
    // Special case: If there's only one row, no operations needed
    if grid_size == 1 {
        return 0;
    }

    // Create a copy of the original grid for calculations
    let mut cal_grid = grid.clone();
    let mut ans = 0;
    
    // Iterate through each column
    for i in 0..grid[0].len() {
        // For each column, ensure values strictly increase from top to bottom
        for j in 1..grid_size {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // If current value is not strictly greater than value above it,
                // calculate operations needed to make it greater
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
    
    // Read grid dimensions (rows and columns)
    let dimensions = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = dimensions
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let grid_size = parts[0];
    let grid_col_size = parts[1];
    
    // Initialize and populate the grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        grid.push(row);
    }
    
    // Calculate and output the result
    let result = minimum_operations(&grid);
    println!("{}", result);
}