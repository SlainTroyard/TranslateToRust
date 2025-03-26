use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 {
            return 0;
        }
        let mut ans = 0;
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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the dimensions of the grid
    stdin.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let m: usize = iter.next().unwrap().parse().expect("Invalid input");
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Initialize the grid
    let mut grid = vec![vec![0; n]; m];

    // Read the grid elements
    for i in 0..m {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        let mut row_iter = buffer.split_whitespace();
        for j in 0..n {
            grid[i][j] = row_iter.next().unwrap().parse().expect("Invalid input");
        }
    }

    // Compute the result
    let result = Solution::minimum_operations(&mut grid);

    // Output the result
    writeln!(stdout, "{}", result).expect("Failed to write result");
}