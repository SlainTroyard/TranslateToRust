use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
        let mut cal_grid = grid.clone();
        let mut ans = 0;
        let m = cal_grid.len();
        let n = cal_grid[0].len();
        
        if m == 1 {
            return 0;
        }
        
        for i in 0..n {
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j-1][i] {
                    ans += cal_grid[j-1][i] + 1 - cal_grid[j][i];
                    cal_grid[j][i] = cal_grid[j-1][i] + 1;
                }
            }
        }
        
        ans
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read dimensions (m rows and n columns)
    let dimensions = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = dimensions
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let m = dimensions[0];
    let n = dimensions[1];
    
    // Create and populate the grid
    let mut grid = vec![vec![0; n]; m];
    for i in 0..m {
        let row = lines.next().unwrap().unwrap();
        let values: Vec<i32> = row
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..n {
            grid[i][j] = values[j];
        }
    }
    
    // Call the solution and print result
    println!("{}", Solution::minimum_operations(&grid));
}