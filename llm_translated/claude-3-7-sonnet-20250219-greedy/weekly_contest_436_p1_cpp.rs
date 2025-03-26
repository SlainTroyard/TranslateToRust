use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn sort_matrix(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Sort diagonals starting from the first column
        for i in 0..n {
            let mut vec = Vec::new();
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        
        // Sort diagonals starting from the first row (excluding the first element)
        for j in 1..n {
            let mut vec = Vec::new();
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            vec.sort(); // Sort in ascending order
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }
        
        grid.clone()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the matrix
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Failed to parse n");
    
    // Initialize the grid
    let mut grid = vec![vec![0; n]; n];
    
    // Read the matrix values
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse grid value"))
            .collect();
        
        for j in 0..n {
            grid[i][j] = values[j];
        }
    }
    
    // Process the matrix
    let result = Solution::sort_matrix(&mut grid);
    
    // Print the result
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
    
    Ok(())
}