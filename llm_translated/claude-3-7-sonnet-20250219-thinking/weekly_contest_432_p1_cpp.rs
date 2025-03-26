use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut result = Vec::new();
        
        let mut cnt = 0;
        for i in 0..n {
            if i & 1 == 1 {
                // Odd row - traverse from right to left
                for j in (0..m).rev() {
                    if cnt == 0 {
                        result.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                // Even row - traverse from left to right
                for j in 0..m {
                    if cnt == 0 {
                        result.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        
        result
    }
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
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    for i in 0..grid_size {
        let row_str = lines.next().unwrap()?;
        let mut values = row_str.split_whitespace();
        for j in 0..grid_col_size {
            grid[i][j] = values.next().unwrap().parse().unwrap();
        }
    }
    
    // Compute zigzag traversal
    let solution = Solution;
    let ans = solution.zigzag_traversal(&grid);
    
    // Output the result
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    
    Ok(())
}