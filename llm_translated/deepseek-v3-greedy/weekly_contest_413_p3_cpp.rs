use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let grid_col_size: usize = line.trim().parse().unwrap();
        let mut row = Vec::with_capacity(grid_col_size);
        for num in line.split_whitespace().skip(1) {
            row.push(num.parse().unwrap());
        }
        grid.push(row);
    }

    // Create solution and compute max score
    let sol = Solution::new();
    println!("{}", sol.max_score(grid));
}

struct Solution;

impl Solution {
    pub fn new() -> Self {
        Solution
    }

    pub fn max_score(&self, grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();

        // Populate pos with bitmask for each number
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all unique numbers
        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();

        // Initialize memoization table
        let mut memo = vec![vec![-1; 1 << m]; n];

        // Define the DFS function
        fn dfs(
            all_nums: &Vec<i32>,
            pos: &HashMap<i32, i32>,
            memo: &mut Vec<Vec<i32>>,
            i: isize,
            j: i32,
        ) -> i32 {
            if i < 0 {
                return 0;
            }
            let i = i as usize;
            if memo[i][j as usize] != -1 {
                return memo[i][j as usize];
            }
            let mut res = dfs(all_nums, pos, memo, i as isize - 1, j);
            let x = all_nums[i];
            let mut t = *pos.get(&x).unwrap_or(&0);
            while t != 0 {
                let lb = t & -t;
                if (j & lb) == 0 {
                    res = res.max(dfs(all_nums, pos, memo, i as isize - 1, j | lb) + x);
                }
                t ^= lb;
            }
            memo[i][j as usize] = res;
            res
        }

        // Start DFS from the last number and empty mask
        dfs(&all_nums, &pos, &mut memo, n as isize - 1, 0)
    }
}