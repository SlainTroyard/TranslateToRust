use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_col_size: usize) -> Vec<i32> {
    let grid_size = grid.len();
    // Calculate the size of the result array as in the original C code
    let result_size = ((grid_col_size * grid_size) + 1) >> 1;
    let mut ans = Vec::with_capacity(result_size);
    let c1 = grid_col_size - 1 - (grid_col_size & 1);
    
    for r in 0..grid_size {
        if r & 1 == 1 {
            // Odd rows: traverse from c1 down to 0 in steps of 2
            let mut c = c1;
            while c < grid_col_size {  // Using while loop to avoid underflow issues
                ans.push(grid[r][c]);
                if c < 2 { break; }  // Prevent underflow
                c -= 2;
            }
        } else {
            // Even rows: traverse from 0 up to grid_col_size in steps of 2
            for c in (0..grid_col_size).step_by(2) {
                ans.push(grid[r][c]);
            }
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid dimensions
    let dimensions = lines.next().unwrap()?;
    let mut iter = dimensions.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().unwrap();
    let grid_col_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read grid values
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Perform zigzag traversal
    let ans = zigzag_traversal(&grid, grid_col_size);
    
    // Print result
    for (i, val) in ans.iter().enumerate() {
        print!("{}", val);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    
    Ok(())
}