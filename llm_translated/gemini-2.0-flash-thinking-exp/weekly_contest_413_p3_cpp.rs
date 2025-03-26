use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

struct Solution {}

impl Solution {
    pub fn max_score(grid: Vec<Vec<int>>) -> int {
        let mut pos: HashMap<int, int> = HashMap::new();
        let m = grid.len();
        for i in 0..m {
            for &x in &grid[i] {
                let entry = pos.entry(x).or_insert(0);
                *entry |= 1 << i;
            }
        }

        let mut all_nums: Vec<int> = Vec::new();
        for (&x, _) in &pos {
            all_nums.push(x);
        }

        let n = all_nums.len();
        let mut memo: Vec<Vec<int>> = vec![vec![-1; 1 << m]; n];

        let mut dfs = |dfs: &dyn Fn(&dyn Fn(&dyn Fn(int, int) -> int, int, int) -> int, int, int) -> int, i: int, j: int| -> int {
            if i < 0 {
                return 0;
            }
            let res_ptr = &mut memo[i as usize][j as usize];
            if *res_ptr != -1 {
                return *res_ptr;
            }

            let mut res = dfs(dfs, i - 1, j); // Option to not use current number
            let x = all_nums[i as usize];
            let t_init = *pos.get(&x).unwrap_or(&0);
            let mut t = t_init;
            while t > 0 {
                let lb = t & -t; // Get least significant bit
                t ^= lb; // Remove least significant bit
                if (j & lb) == 0 {
                    res = std::cmp::max(res, dfs(dfs, i - 1, j | lb) + x); // Option to use current number
                }
            }
            *res_ptr = res;
            res
        };
        dfs(&dfs, (n - 1) as int, 0)
    }
}

fn main() -> io::Result<()> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();

    reader.read_line(&mut input)?;
    let grid_size: usize = input.trim().parse().unwrap();

    let mut grid: Vec<Vec<int>> = Vec::new();
    for _ in 0..grid_size {
        input.clear();
        reader.read_line(&mut input)?;
        let grid_col_size: usize = input.trim().parse().unwrap();

        let mut row: Vec<int> = Vec::new();
        input.clear();
        reader.read_line(&mut input)?;
        let nums_str: Vec<&str> = input.trim().split_whitespace().collect();
        for j in 0..grid_col_size {
            row.push(nums_str[j].parse().unwrap());
        }
        grid.push(row);
    }

    let sol = Solution {};
    println!("{}", sol.max_score(grid));

    Ok(())
}