use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Process diagonals starting from the main diagonal and moving down
        for i in 0..n {
            let mut vec = Vec::new();
            // Collect values from the diagonal
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            // Sort in descending order
            vec.sort_by(|a, b| b.cmp(a));
            // Put sorted values back
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        
        // Process diagonals starting from the column 1 and moving right
        for j in 1..n {
            let mut vec = Vec::new();
            // Collect values from the diagonal
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            // Sort in ascending order
            vec.sort();
            // Put sorted values back
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }
        
        grid
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the matrix size
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize the grid
    let mut grid = vec![vec![0; n]; n];
    
    // Read the matrix values
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        for j in 0..n {
            grid[i][j] = values[j];
        }
    }
    
    // Solve the problem
    let solution = Solution;
    let result = Solution::sort_matrix(grid);
    
    // Print the result
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}