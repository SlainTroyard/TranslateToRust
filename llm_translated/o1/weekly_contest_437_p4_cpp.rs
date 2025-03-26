// Weekly Contest 437 Problem 4
// Translated from C++ to Rust

use std::io::{self, Read};

// A simple struct to encapsulate our solution method
struct Solution;

impl Solution {
    /// Returns the maximal length of a 'V'-shaped diagonal as described
    /// in the original C++ code. The logic is preserved exactly.
    fn len_of_v_diagonal(&self, grid: &Vec<Vec<i32>>) -> i32 {
        let m = grid.len();         // Number of rows
        let n = grid[0].len();      // Number of columns

        // The four directions used in the DFS.
        // Equivalent to DIRS in the original C++ code:
        // { {1,1}, {1,-1}, {-1,-1}, {-1,1} }
        const DIRS: [(i32, i32); 4] = [(1,1), (1,-1), (-1,-1), (-1,1)];

        // 4D memo array:
        // memo[i][j][k][turn_state]
        // i, j: cell indices
        // k: which direction from DIRS
        // turn_state: 0 (cannot turn anymore) or 1 (can still turn)
        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];

        // A helper function that recursively explores valid "diagonal" paths.
        fn dfs(
            i: i32,
            j: i32,
            k: usize,
            can_turn: bool,
            target: i32,
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
            m: usize,
            n: usize,
        ) -> i32 {
            // If out of bounds, return 0
            if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 {
                return 0;
            }

            // Calculate next step in the specified direction k
            const DIRS: [(i32, i32); 4] = [(1,1), (1,-1), (-1,-1), (-1,1)];
            let ni = i + DIRS[k].0;
            let nj = j + DIRS[k].1;

            // If next cell is out of bounds or doesn't have the required value, stop
            if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 || grid[ni as usize][nj as usize] != target {
                return 0;
            }

            let can_turn_idx = if can_turn { 1 } else { 0 };
            let res_ref = &mut memo[ni as usize][nj as usize][k][can_turn_idx];

            // If we already computed this state, just return it
            if *res_ref != 0 {
                return *res_ref;
            }

            // Continue in the same direction, flipping the target (2 - target) 
            // between 1 and 2 for the next step
            *res_ref = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, m, n);

            // If a turn is still allowed, explore turning to the next direction
            // (i.e. (k + 1) % 4) without allowing further turns
            if can_turn {
                let nk = (k + 1) % 4;
                let alt_res = dfs(ni, nj, nk, false, 2 - target, grid, memo, m, n);
                if alt_res > *res_ref {
                    *res_ref = alt_res;
                }
            }

            // Increment the result to include this cell in the count
            *res_ref += 1;
            *res_ref
        }

        // Try starting from every cell that has value 1,
        // exploring in all four directions with the possibility of turning once
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        let length = dfs(i as i32, j as i32, k, true, 2, grid, &mut memo, m, n) + 1;
                        if length > ans {
                            ans = length;
                        }
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    // Exactly preserve the original C++ I/O format:
    // 1) Read two integers n, m
    // 2) Read n rows of m integers each
    // 3) Print the result of the solution
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read n (number of rows) and m (number of columns)
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    // Build the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            grid[i][j] = tokens.next().unwrap().parse().unwrap();
        }
    }

    // Solve and output the answer
    let sol = Solution;
    let ans = sol.len_of_v_diagonal(&grid);
    println!("{}", ans);
}