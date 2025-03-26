use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();
        for i in 0..m {
            for &x in &grid[i] {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        let mut all_nums: Vec<i32> = pos.keys().cloned().collect();
        all_nums.sort_unstable();

        let n = all_nums.len();
        let mut memo = vec![vec![-1; 1 << m]; n];

        fn dfs(
            all_nums: &[i32],
            pos: &HashMap<i32, i32>,
            memo: &mut Vec<Vec<i32>>,
            i: usize,
            j: i32,
        ) -> i32 {
            if i == 0 {
                return 0;
            }
            let i = i - 1;
            if memo[i][j as usize] != -1 {
                return memo[i][j as usize];
            }
            memo[i][j as usize] = dfs(all_nums, pos, memo, i, j);
            let x = all_nums[i];
            let mut t = *pos.get(x).unwrap();
            while t != 0 {
                let lb = t & -t;
                if j & lb == 0 {
                    memo[i][j as usize] = memo[i][j as usize].max(
                        dfs(all_nums, pos, memo, i, j | lb) + x,
                    );
                }
                t ^= lb;
            }
            memo[i][j as usize]
        }

        dfs(&all_nums, &pos, &mut memo, n, 0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let grid_size: usize = lines.next().unwrap()?.parse().unwrap();
    let mut grid = Vec::with_capacity(grid_size);

    for _ in 0..grid_size {
        let grid_col_size: usize = lines.next().unwrap()?.parse().unwrap();
        let mut row = Vec::with_capacity(grid_col_size);
        for _ in 0..grid_col_size {
            let num: i32 = lines.next().unwrap()?.parse().unwrap();
            row.push(num);
        }
        grid.push(row);
    }

    let result = Solution::max_score(&grid);
    println!("{}", result);

    Ok(())
}