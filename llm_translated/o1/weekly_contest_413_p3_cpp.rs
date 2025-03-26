// LeetCode Weekly Contest 413 Problem 3 in Rust
// 
// REQUIREMENTS MET:
// 1. Translated the ENTIRE file as a complete program, including main()
// 2. Preserved the algorithm logic exactly
// 3. Used idiomatic Rust with proper error handling
// 4. Maintained the EXACT SAME stdin/stdout format
// 5. Followed the input parsing logic carefully to match the C++ code.
//
// NOTE: The input format is:
//   gridSize
//   gridColSize (for row 0)
//   col_0_0
//   col_0_1
//   ...
//   col_0_{gridColSize-1}
//   gridColSize (for row 1)
//   col_1_0
//   ...
//   etc.
// Output is a single integer (the result) on its own line.
//
// To replicate the flexible whitespace behavior of C++'s "cin >> ...",
// we use a simple Scanner that reads tokens from stdin.

use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

// A small utility to read space-separated tokens from stdin incrementally
struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            tokens: VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        // If our buffer is empty, read a new line and split it into tokens
        while self.tokens.is_empty() {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            for token in line.split_whitespace() {
                self.tokens.push_back(token.to_string());
            }
        }
        // Pop front token and parse it
        self.tokens
            .pop_front()
            .unwrap()
            .parse::<T>()
            .expect("Failed to parse token")
    }
}

// We define a "Solution" struct to mirror the C++ class style
struct Solution;

impl Solution {
    fn max_score(&self, grid: &[Vec<i32>]) -> i32 {
        // Map each integer x to a bitmask, indicating which row(s) contain x
        let mut pos = HashMap::new();
        let m = grid.len();
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                let entry = pos.entry(x).or_insert(0);
                *entry |= 1 << i;
            }
        }

        // Collect all distinct numbers that appear in the grid
        let mut all_nums = Vec::new();
        for &key in pos.keys() {
            all_nums.push(key);
        }

        let n = all_nums.len();

        // memo[i][j] will store the maximum score using all_nums[..=i]
        // when we've already used the rows indicated by the bitmask j
        let mut memo = vec![vec![-1; 1 << m]; n];

        // DFS function (equivalent to the C++ lambda) with recursion
        fn dfs(
            i: isize,        // index in all_nums (may be -1 for base case)
            j: usize,        // bitmask of used rows
            all_nums: &[i32],
            pos: &HashMap<i32, i32>,
            memo: &mut [Vec<i32>],
            m: usize,
        ) -> i32 {
            if i < 0 {
                // Base case: no numbers left
                return 0;
            }
            let i_usize = i as usize;
            if memo[i_usize][j] != -1 {
                // Already computed
                return memo[i_usize][j];
            }

            // Skip using the current number
            let mut res = dfs(i - 1, j, all_nums, pos, memo, m);

            // Try using the current number x in any row that is still free
            let x = all_nums[i_usize];
            let mut t = *pos.get(&x).unwrap_or(&0);
            while t != 0 {
                let lb = t & -t; // lowest set bit
                t ^= lb;         // remove that bit
                if (j as i32 & lb) == 0 {
                    let new_j = j | lb as usize;
                    let candidate = dfs(i - 1, new_j, all_nums, pos, memo, m) + x;
                    if candidate > res {
                        res = candidate;
                    }
                }
            }

            memo[i_usize][j] = res;
            res
        }

        dfs((n as isize) - 1, 0, &all_nums, &pos, &mut memo, m)
    }
}

fn main() {
    // Use our scanner to read tokens from stdin
    let mut sc = Scanner::new();

    // Read the number of rows in the grid
    let grid_size: usize = sc.next();
    // Initialize grid
    let mut grid = vec![Vec::new(); grid_size];

    // Read each row
    for i in 0..grid_size {
        let grid_col_size: usize = sc.next();
        let mut row = Vec::with_capacity(grid_col_size);
        for _ in 0..grid_col_size {
            row.push(sc.next());
        }
        grid[i] = row;
    }

    // Solve
    let sol = Solution;
    let answer = sol.max_score(&grid);

    // Print result exactly as in the original C++ code
    println!("{}", answer);
}