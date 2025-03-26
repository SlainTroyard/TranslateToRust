// LeetCode Weekly Contest 438 Problem 2 in Rust
// Requirements:
// 1) Translates the entire C++ program to Rust, including main function and I/O
// 2) Preserves the algorithm logic exactly
// 3) Uses idiomatic Rust
// 4) Maintains the exact same stdin/stdout format as the original code
// 5) Contains only the Rust code in this code block

use std::collections::BinaryHeap;
use std::io::{self, Read};

struct Solution;

impl Solution {
    /// Calculates the maximum sum by picking up to k values from rows in descending order,
    /// subject to each row's limit.
    fn max_sum(&self, grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, mut k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        // Prepare a max-heap; each element is (value, row_index, col_index).
        let mut pq = BinaryHeap::new();

        // Sort each row in descending order and push the first element of each row into the heap.
        for (i, row) in grid.iter_mut().enumerate() {
            row.sort_unstable_by(|a, b| b.cmp(a));
            // Push (value, row, column)
            pq.push((row[0], i, 0));
        }

        let mut ans = 0_i64;

        // Extract up to k elements, skipping those that exceed the row's limit.
        while k > 0 && !pq.is_empty() {
            let (val, r, c) = pq.pop().unwrap();
            // If the index is already at/above the limit, skip.
            if c as i32 >= limits[r] {
                continue;
            }
            ans += val as i64;
            k -= 1;
            // Push the next element from this row if available.
            if c + 1 < m {
                pq.push((grid[r][c + 1], r, c + 1));
            }
        }
        ans
    }
}

fn main() {
    // Read all input as a single string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Tokenize based on whitespace.
    let mut tokens = input.split_whitespace();

    // Read n, m, k.
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    // Read the grid.
    let mut grid = vec![vec![0_i32; m]; n];
    for i in 0..n {
        for j in 0..m {
            grid[i][j] = tokens.next().unwrap().parse().unwrap();
        }
    }

    // Read limits array.
    let mut limits = vec![0_i32; n];
    for i in 0..n {
        limits[i] = tokens.next().unwrap().parse().unwrap();
    }

    // Compute and print the result.
    let sol = Solution;
    let result = sol.max_sum(&mut grid, &limits, k);
    println!("{}", result);
}