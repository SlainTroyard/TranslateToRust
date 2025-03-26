use std::io::{self, BufRead};
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
        let mut pos = HashMap::new();
        let m = grid.len();
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();
        let mut memo = vec![vec![-1; 1 << m]; n];

        let dfs = |dfs: &dyn Fn(&dyn Fn(&dyn Fn(&[i32], usize, usize) -> i32, &[i32], usize, usize) -> i32, &[i32], usize, usize) -> i32, nums: &[i32], i: usize, j: usize| -> i32 {
            if i == 0 {
                return 0;
            }
            let x = nums[i - 1];
            if memo[i - 1][j] != -1 {
                return memo[i - 1][j];
            }
            let mut res = dfs(dfs, nums, i - 1, j);
            let t = pos[&x];
            let mut lb = 1;
            while t > 0 {
                lb = t & -t;
                if (j & lb) == 0 {
                    res = res.max(dfs(dfs, nums, i - 1, j | lb) + x);
                }
                t ^= lb;
            }
            memo[i - 1][j] = res;
            res
        };

        dfs(&dfs, &all_nums, n, 0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows
    let grid_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        // Read the number of columns and the row values
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let grid_col_size: usize = iter.next().unwrap().parse().unwrap();
        let row: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();
        assert_eq!(row.len(), grid_col_size);
        grid.push(row);
    }

    // Solve the problem
    let sol = Solution;
    let result = sol.max_score(&grid);

    // Output the result
    println!("{}", result);
}