use std::io::{self, BufRead};
use std::cmp::Reverse;

struct Solution;

impl Solution {
    fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Process diagonals starting from the leftmost column (i, 0)
        for i in 0..n {
            let mut vec = Vec::new();
            // Extract elements from the diagonal starting at (i, 0)
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            // Sort in descending order (greater<int> in C++)
            vec.sort_by_key(|&x| Reverse(x));
            // Put elements back
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        
        // Process diagonals starting from the top row (0, j) where j > 0
        for j in 1..n {
            let mut vec = Vec::new();
            // Extract elements from the diagonal starting at (0, j)
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            // Sort in ascending order
            vec.sort();
            // Put elements back
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }
        
        grid
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    // Read the size of the grid
    let n: usize = input.next().unwrap()?.trim().parse()
        .expect("Failed to parse n");
    
    // Initialize and read the grid
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let line = input.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse grid value"))
            .collect();
        
        for j in 0..n {
            grid[i][j] = values[j];
        }
    }
    
    // Process the grid
    let result = Solution::sort_matrix(grid);
    
    // Print the result with the exact same format as the original code
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
    
    Ok(())
}