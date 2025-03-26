use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn sort_matrix(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Sort diagonals from top-left to bottom-right
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
        
        // Sort diagonals from top-right to bottom-left
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

    // Read the size of the grid
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![vec![0; n]; n];

    // Read the grid values
    for i in 0..n {
        let row: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid[i] = row;
    }

    // Sort the matrix
    let result = Solution::sort_matrix(&mut grid);

    // Print the result
    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    Ok(())
}