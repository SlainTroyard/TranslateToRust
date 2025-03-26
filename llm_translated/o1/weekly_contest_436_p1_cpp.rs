use std::io::{self, BufRead};

// A simple struct representing our solution, similar to the C++ 'class Solution'.
struct Solution;

impl Solution {
    /// Sorts the matrix based on the specified diagonal sorting logic:
    /// - For diagonals starting in the first column, sort in descending order.
    /// - For diagonals starting in the top row (excluding the very first cell), sort in ascending order.
    fn sort_matrix(&mut self, mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        // Sort diagonals starting from the first column in descending order
        for i in 0..n {
            let mut diagonal = Vec::new();
            let mut k = 0;
            while i + k < n {
                diagonal.push(grid[i + k][k]);
                k += 1;
            }
            diagonal.sort_by(|a, b| b.cmp(a)); // descending order
            let mut k = 0;
            while i + k < n {
                grid[i + k][k] = diagonal[k];
                k += 1;
            }
        }

        // Sort diagonals starting from the second column in ascending order
        for j in 1..n {
            let mut diagonal = Vec::new();
            let mut k = 0;
            while j + k < n {
                diagonal.push(grid[k][j + k]);
                k += 1;
            }
            diagonal.sort(); // ascending order
            let mut k = 0;
            while j + k < n {
                grid[k][j + k] = diagonal[k];
                k += 1;
            }
        }

        grid
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare to read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the matrix
    let n_str = lines
        .next()
        .ok_or("EOF reached while reading matrix size 'n'")??;
    let n: usize = n_str.trim().parse()?;

    // Read exactly n*n integers, regardless of how they're spaced or split across lines
    let mut numbers = Vec::with_capacity(n * n);
    while numbers.len() < n * n {
        let line = lines
            .next()
            .ok_or("Not enough input provided for the matrix")??;
        for token in line.split_whitespace() {
            numbers.push(token.parse()?);
            if numbers.len() == n * n {
                break;
            }
        }
    }

    // Build the 2D grid
    let mut grid = vec![vec![0; n]; n];
    let mut idx = 0;
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = numbers[idx];
            idx += 1;
        }
    }

    // Create a solution instance and process the grid
    let mut solution = Solution;
    let result = solution.sort_matrix(grid);

    // Print the resulting matrix in the same format as the C++ code
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }

    Ok(())
}