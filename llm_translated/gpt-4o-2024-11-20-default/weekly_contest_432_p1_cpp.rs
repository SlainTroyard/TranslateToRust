use std::io::{self, BufRead};

// Define the solution struct similar to the class in C++.
struct Solution;

impl Solution {
    // `zigzag_traversal` is the equivalent function to `zigzagTraversal` in the C++ code.
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len(); // Number of rows in the grid
        let m = grid[0].len(); // Number of columns in the grid
        let mut vec = Vec::new();
        let mut cnt = 0; // Counter to track the alternating condition

        for i in 0..n {
            if i & 1 == 1 {
                // For odd rows, traverse from right to left
                for j in (0..m).rev() {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                // For even rows, traverse from left to right
                for j in 0..m {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        vec
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line contains the size of the grid (gridSize and gridColSize)
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let grid_size: usize = dimensions.next().unwrap().parse().unwrap();
    let grid_col_size: usize = dimensions.next().unwrap().parse().unwrap();

    // Populate the grid from subsequent lines
    let mut grid = Vec::new();
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        grid.push(row);
    }

    // Create an instance of the Solution and solve the problem
    let solution = Solution;
    let ans = solution.zigzag_traversal(grid);

    // Print the output as space-separated integers
    for (i, val) in ans.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!(); // Ensure a newline at the end of the output
}