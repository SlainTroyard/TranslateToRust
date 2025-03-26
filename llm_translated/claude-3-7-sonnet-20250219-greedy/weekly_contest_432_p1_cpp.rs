use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        
        let mut cnt = 0;
        for i in 0..n {
            if i & 1 == 1 {
                // Odd rows: traverse right to left
                for j in (0..m).rev() {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                // Even rows: traverse left to right
                for j in 0..m {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        
        vec
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid dimensions
    let dimensions = lines
        .next()
        .expect("Failed to read dimensions")
        .expect("Failed to parse dimensions");
    
    let dimensions: Vec<usize> = dimensions
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse dimension as number"))
        .collect();
    
    let grid_size = dimensions[0];
    let grid_col_size = dimensions[1];
    
    // Read grid values
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    for i in 0..grid_size {
        let row = lines
            .next()
            .expect("Failed to read grid row")
            .expect("Failed to parse grid row");
        
        let values: Vec<i32> = row
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse grid value as number"))
            .collect();
        
        for j in 0..grid_col_size {
            grid[i][j] = values[j];
        }
    }
    
    // Solve and output result
    let solution = Solution;
    let ans = Solution::zigzag_traversal(&grid);
    
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    
    Ok(())
}