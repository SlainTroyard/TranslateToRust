// Translation of LeetCode Weekly Contest 413 Problem 3 from C to Rust
// Requirements Met:
// 1) Entire file translated, including main and I/O
// 2) Algorithm logic preserved exactly
// 3) Idiomatic Rust with error handling
// 4) Same stdin/stdout format
// 5) Comments added for clarity

use std::io::{self, BufRead};
use std::error::Error;
use std::cmp::max;

// A simple scanner to parse space-separated integers from stdin
struct Scanner<B> {
    reader: B,
    buffer: Vec<String>,
}

impl<B: BufRead> Scanner<B> {
    fn new(reader: B) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }

    // Reads the next token (space-separated) and tries to parse it into type T
    fn token<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn Error>> {
        loop {
            // If our buffer already has some tokens, use them
            if let Some(front) = self.buffer.pop() {
                return Ok(front.parse()?);
            }
            // Otherwise, read a new line and split it into tokens
            let mut input_line = String::new();
            if self.reader.read_line(&mut input_line)? == 0 {
                return Err("Unexpected EOF".into());
            }
            self.buffer = input_line
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

// Translated function: calculates max score based on given grid
fn max_score(grid: &[Vec<i32>], grid_size: usize, grid_col_size: &[usize]) -> i32 {
    let mut ans = 0;
    let mut maxnum = 0;
    let m = grid_size;
    // n = number of columns in the first row, following the original code
    let n = grid_col_size[0];

    // Find the maximum number in the entire grid
    for i in 0..m {
        for j in 0..n {
            maxnum = max(maxnum, grid[i][j]);
        }
    }
    let maxnum_usize = maxnum as usize;

    // numsRaw[x] accumulates bit masks indicating in which rows the value x appears
    let mut nums_raw = vec![0; maxnum_usize + 1];
    for i in 0..m {
        for j in 0..n {
            let val = grid[i][j] as usize;
            nums_raw[val] |= 1 << i;
        }
    }

    // Prepare dp array: dp[i][mask] = best score using values up to i with row-allocation "mask"
    // Dimensions: (maxnum + 1) x (1 << (m+1)) to match the original code
    let dp_width = 1 << (m + 1);
    let mut dp = vec![vec![i32::MIN; dp_width]; maxnum_usize + 1];
    dp[0][0] = 0;

    // Main DP loop
    for i in 1..=maxnum_usize {
        for j in 0..(1 << m) {
            // Option 1: skip using value i
            dp[i][j] = max(dp[i][j], dp[i - 1][j]);
            // Option 2: use value i in any row k if possible
            for k in 0..m {
                if ((nums_raw[i] >> k) & 1) == 1 && ((j >> k) & 1) == 1 {
                    let candidate = dp[i - 1][j ^ (1 << k)] + i as i32;
                    dp[i][j] = max(dp[i][j], candidate);
                    ans = max(ans, dp[i][j]);
                }
            }
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    // Read gridSize
    let grid_size: usize = scan.token()?;

    // Read the array gridColSize
    let mut grid_col_size = vec![0_usize; grid_size];
    for i in 0..grid_size {
        grid_col_size[i] = scan.token()?;
    }

    // Read the grid data
    let mut grid = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        let mut row = Vec::with_capacity(grid_col_size[i]);
        for _ in 0..grid_col_size[i] {
            let val: i32 = scan.token()?;
            row.push(val);
        }
        grid.push(row);
    }

    // Compute the answer
    let ans = max_score(&grid, grid_size, &grid_col_size);

    // Print the result
    println!("{}", ans);

    Ok(())
}