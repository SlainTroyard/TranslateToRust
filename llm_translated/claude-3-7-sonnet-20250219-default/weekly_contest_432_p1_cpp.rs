use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        
        let mut cnt = 0;
        for i in 0..n {
            if i & 1 == 1 {
                // Odd row: go from right to left
                for j in (0..m).rev() {
                    cnt ^= 1;
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                }
            } else {
                // Even row: go from left to right
                for j in 0..m {
                    cnt ^= 1;
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                }
            }
        }
        
        vec
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse grid dimensions
    let dimensions = lines.next().unwrap()?;
    let mut dims = dimensions.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let grid_size = dims.next().unwrap();
    let grid_col_size = dims.next().unwrap();
    
    // Parse grid values
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    for i in 0..grid_size {
        let row_input = lines.next().unwrap()?;
        let mut values = row_input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        
        for j in 0..grid_col_size {
            grid[i][j] = values.next().unwrap();
        }
    }
    
    // Solve and output the result
    let solution = Solution{};
    let ans = Solution::zigzag_traversal(&grid);
    
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    
    Ok(())
}