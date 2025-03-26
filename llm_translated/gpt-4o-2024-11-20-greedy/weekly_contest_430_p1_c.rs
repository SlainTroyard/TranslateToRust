use std::io::{self, BufRead};
use std::vec::Vec;

// Translated function to compute minimum operations
fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    let grid_col_size = if !grid.is_empty() { grid[0].len() } else { 0 };
    
    let mut ans = 0;

    // Early return for 1-row grid
    if grid_size == 1 {
        return 0;
    }

    // Iterate over columns
    for col in 0..grid_col_size {
        for row in 1..grid_size {
            if grid[row][col] <= grid[row - 1][col] {
                ans += grid[row - 1][col] + 1 - grid[row][col];
                grid[row][col] = grid[row - 1][col] + 1;
            }
        }
    }

    ans
}

fn main() {
    // Initialize input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read gridSize and gridColSize
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let grid_size = dimensions[0];
    let grid_col_size = dimensions[1];

    // Initialize the grid
    let mut grid = Vec::with_capacity(grid_size);

    // Read each row of the grid
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Ensure the grid dimensions are valid
    assert_eq!(grid.len(), grid_size);
    for row in &grid {
        assert_eq!(row.len(), grid_col_size);
    }

    // Call the translated function
    let result = minimum_operations(&mut grid);

    // Output the result to stdout
    println!("{}", result);
}