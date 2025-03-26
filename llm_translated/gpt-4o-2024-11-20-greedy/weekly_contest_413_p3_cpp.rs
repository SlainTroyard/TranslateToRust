use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos = HashMap::new();
        let m = grid.len();

        // Populate the `pos` map
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all unique numbers
        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();

        // Memoization table
        let mut memo = vec![vec![-1; 1 << m]; n];

        // Recursive DFS function with memoization
        fn dfs(
            dfs: &dyn Fn(&dyn Fn(&dyn Fn(i32, i32) -> i32, i32, i32) -> i32, i32, i32) -> i32,
            memo: &mut Vec<Vec<i32>>,
            all_nums: &[i32],
            pos: &HashMap<i32, i32>,
            i: i32,
            j: i32,
        ) -> i32 {
            if i < 0 {
                return 0;
            }
            let i_usize = i as usize;
            let j_usize = j as usize;
            if memo[i_usize][j_usize] != -1 {
                return memo[i_usize][j_usize];
            }
            let mut res = dfs(dfs, memo, all_nums, pos, i - 1, j);
            let x = all_nums[i_usize];
            let mut t = pos[&x];
            while t != 0 {
                let lb = t & -t; // Extract the lowest bit
                if (j & lb) == 0 {
                    res = res.max(dfs(dfs, memo, all_nums, pos, i - 1, j | lb) + x);
                }
                t ^= lb; // Remove the lowest bit
            }
            memo[i_usize][j_usize] = res;
            res
        }

        dfs(&dfs, &mut memo, &all_nums, &pos, (n - 1) as i32, 0)
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size
    let grid_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse grid size");

    // Read the grid
    let mut grid = Vec::new();
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse grid element"))
            .collect();
        grid.push(row);
    }

    // Solve the problem
    let result = Solution::max_score(grid);

    // Print the result
    println!("{}", result);
}