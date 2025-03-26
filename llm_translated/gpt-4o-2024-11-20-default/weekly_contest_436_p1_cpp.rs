// Problem: Weekly Contest 436 Problem 1
use std::io::{self, BufRead};
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Sort diagonals starting in the first column
        for i in 0..n {
            let mut vec = Vec::new();
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            vec.sort_unstable_by_key(|&x| Reverse(x)); // Sort in descending order
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        
        // Sort diagonals starting in the first row (excluding the main diagonal)
        for j in 1..n {
            let mut vec = Vec::new();
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            vec.sort_unstable(); // Sort in ascending order
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }

        grid
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read 'n' value
    let n: usize = iterator
        .next()
        .expect("Expected input for n")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n as usize");

    // Parse the grid values
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let line = iterator
            .next()
            .expect("Expected input for grid row")
            .unwrap();
        grid[i] = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse grid value as i32"))
            .collect();
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.sort_matrix(grid);

    // Output the result to stdout
    for row in result {
        println!(
            "{}",
            row.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}