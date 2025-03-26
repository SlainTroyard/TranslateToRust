use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    // Calculate the size of the result array: approximately half of all grid elements
    let result_size = (grid_col_size * grid_size + 1) >> 1;
    let mut ans = Vec::with_capacity(result_size);
    
    // The starting column for odd rows, adjusted for even/odd column count
    let c1 = grid_col_size - 1 - (grid_col_size & 1);
    
    for r in 0..grid_size {
        if r & 1 == 1 {
            // For odd rows, start from c1 and move left by 2
            let mut c = c1;
            while c < grid_col_size { // This check handles unsigned underflow
                ans.push(grid[r][c]);
                if c < 2 { break; }
                c -= 2;
            }
        } else {
            // For even rows, start from 0 and move right by 2
            let mut c = 0;
            while c < grid_col_size {
                ans.push(grid[r][c]);
                c += 2;
            }
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid dimensions
    let dimensions = lines.next().unwrap().unwrap();
    let mut iter = dimensions.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().unwrap();
    let grid_col_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read grid values
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Perform zigzag traversal
    let ans = zigzag_traversal(&grid, grid_size, grid_col_size);
    
    // Print the result
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}