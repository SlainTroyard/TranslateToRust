use std::io::{self, BufRead};

fn zigzag_traversal(grid: Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    // Result vector to store the zigzag traversal
    let mut ans = Vec::with_capacity((grid_size * grid_col_size + 1) / 2);
    
    // Iterate through each row
    for r in 0..grid_size {
        if r % 2 == 1 {
            // Odd row: traverse from right to left, skipping every other column
            for c in (0..grid_col_size).rev().step_by(2) {
                ans.push(grid[r][c]);
            }
        } else {
            // Even row: traverse from left to right, skipping every other column
            for c in (0..grid_col_size).step_by(2) {
                ans.push(grid[r][c]);
            }
        }
    }
    
    ans
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid size and column size
    let first_line = lines.next().unwrap().unwrap();
    let mut sizes = first_line.split_whitespace();
    let grid_size: usize = sizes.next().unwrap().parse().unwrap();
    let grid_col_size: usize = sizes.next().unwrap().parse().unwrap();
    
    // Read the grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();
        grid.push(row);
    }
    
    // Perform the zigzag traversal
    let result = zigzag_traversal(grid, grid_size, grid_col_size);
    
    // Print the result
    for (i, val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}