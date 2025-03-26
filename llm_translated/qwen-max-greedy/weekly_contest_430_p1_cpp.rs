use std::io::{self, BufRead, Write};

fn main() {
    // Read the grid dimensions and values from stdin
    let (m, n, grid) = read_grid();
    // Create a new Solution instance and call minimum_operations
    let solution = Solution;
    let result = solution.minimum_operations(grid);
    // Output the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn minimum_operations(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let m = grid.len();
        if m == 1 {
            return 0;
        }
        let n = grid[0].len();

        for i in 0..n {
            for j in 1..m {
                if grid[j][i] <= grid[j - 1][i] {
                    ans += grid[j - 1][i] + 1 - grid[j][i];
                    grid[j][i] = grid[j - 1][i] + 1;
                }
            }
        }

        ans
    }
}

// Helper function to read the grid from stdin
fn read_grid() -> (usize, usize, Vec<Vec<i32>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the dimensions of the grid
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid with the given dimensions
    let mut grid = vec![vec![0; n]; m];

    // Read the grid values
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse().unwrap();
        }
    }

    (m, n, grid)
}