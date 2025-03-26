use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos = HashMap::new();
        let m = grid.len();

        // Populate `pos` with bitmask representation of row indices
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all numbers from the map keys
        let all_nums: Vec<i32> = pos.keys().copied().collect();
        let n = all_nums.len();

        // Memoization table
        let mut memo = vec![vec![-1; 1 << m]; n];

        // DFS function
        fn dfs(
            dfs: &dyn Fn(&dyn Fn(&dyn Fn(i32, i32) -> i32, i32, i32) -> i32, i32, i32) -> i32,
            i: i32,
            j: i32,
            all_nums: &[i32],
            pos: &HashMap<i32, i32>,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i < 0 {
                return 0;
            }

            let i = i as usize;
            if memo[i][j as usize] != -1 {
                return memo[i][j as usize];
            }

            let mut res = dfs(dfs, i as i32 - 1, j, all_nums, pos, memo);
            let x = all_nums[i];
            let mut t = pos[&x];

            while t != 0 {
                let lb = t & -t; // Extract lowest bit
                t ^= lb; // Remove the lowest bit

                if (j & lb) == 0 {
                    res = res.max(dfs(
                        dfs,
                        i as i32 - 1,
                        j | lb,
                        all_nums,
                        pos,
                        memo,
                    ) + x);
                }
            }

            memo[i][j as usize] = res;
            res
        }

        dfs(
            &dfs,
            n as i32 - 1,
            0,
            &all_nums,
            &pos,
            &mut memo,
        )
    }
}

fn main() {
    // Parse input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of rows in the grid
    let grid_size = lines
        .next()
        .expect("Failed to read grid size")
        .expect("Invalid input")
        .trim()
        .parse::<usize>()
        .expect("Invalid grid size");

    let mut grid = Vec::new();

    for _ in 0..grid_size {
        let line = lines
            .next()
            .expect("Failed to read grid row")
            .expect("Invalid input");
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid grid value"))
            .collect();
        grid.push(row);
    }

    // Solve the problem
    let result = Solution::max_score(grid);

    // Print the result
    println!("{}", result);
}