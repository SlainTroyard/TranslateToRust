use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let m = grid[0].len();
    let mut vec = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if i & 1 == 1 {
            // Traverse from right to left
            for j in (0..m).rev() {
                vec.push(row[j]);
            }
        } else {
            // Traverse from left to right
            for j in 0..m {
                vec.push(row[j]);
            }
        }
    }
    vec
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get gridSize and gridColSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let gridSize: usize = iter.next().unwrap().parse().unwrap();
    let gridColSize: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![vec![0; gridColSize]; gridSize];

    // Fill the grid with values from stdin
    for i in 0..gridSize {
        let line = lines.next().unwrap().unwrap();
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse().unwrap();
        }
    }

    // Perform the zigzag traversal
    let solution = Solution;
    let ans = solution.zigzag_traversal(&grid);

    // Print the result to stdout
    for (i, &value) in ans.iter().enumerate() {
        print!("{}{}", value, if i < ans.len() - 1 { " " } else { "" });
    }
    println!();
}

struct Solution;

impl Solution {
    fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            if i & 1 == 1 {
                // Traverse from right to left
                for j in (0..m).rev() {
                    vec.push(row[j]);
                }
            } else {
                // Traverse from left to right
                for j in 0..m {
                    vec.push(row[j]);
                }
            }
        }
        vec
    }
}