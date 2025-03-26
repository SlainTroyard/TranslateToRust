// Problem: Weekly Contest 430 Problem 1
use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut cal_grid = grid.clone();
        let mut ans = 0;
        let m = cal_grid.len();
        let n = cal_grid[0].len();

        // If there's only one row, no operations are needed
        if m == 1 {
            return 0;
        }

        // Iterate through each column
        for i in 0..n {
            // Iterate through each row starting from the second row
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j - 1][i] {
                    // Calculate the required increment
                    ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                    // Update the current cell to maintain the strictly increasing property
                    cal_grid[j][i] = cal_grid[j - 1][i] + 1;
                }
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the dimensions of the grid (m rows, n columns)
    stdin.read_line(&mut input).unwrap();
    let dimensions: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let m = dimensions[0];
    let n = dimensions[1];

    let mut grid = Vec::new();

    // Read the grid values
    for _ in 0..m {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.minimum_operations(grid);

    // Print the result
    println!("{}", result);
}