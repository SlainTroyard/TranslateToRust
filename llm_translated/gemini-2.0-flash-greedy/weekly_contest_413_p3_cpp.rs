use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_score(&self, grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();

        for i in 0..m {
            for &x in &grid[i] {
                let entry = pos.entry(x).or_insert(0);
                *entry |= 1 << i;
            }
        }

        let mut all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; 1 << m]; n];

        fn dfs(i: usize, j: i32, all_nums: &Vec<i32>, pos: &HashMap<i32, i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == 0 {
                return 0;
            }
            if memo[i][j as usize] != -1 {
                return memo[i][j as usize];
            }

            let mut res = dfs(i - 1, j, all_nums, pos, memo);
            let x = all_nums[i];

            let t = *pos.get(&x).unwrap();
            let mut temp = t;

            while temp != 0 {
                let lb = temp & (-temp);
                if (j & lb) == 0 {
                    res = max(res, dfs(i - 1, j | lb, all_nums, pos, memo) + x);
                }
                temp ^= lb;
            }

            memo[i][j as usize] = res;
            return res;
        }

        if n > 0 {
            dfs(n - 1, 0, &all_nums, &pos, &mut memo)
        } else {
            0
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let grid_size: usize = lines.next().unwrap().parse()?;
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _ in 0..grid_size {
        let grid_col_size: usize = lines.next().unwrap().parse()?;
        let mut row: Vec<i32> = Vec::new();

        let row_values: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for j in 0..grid_col_size {
            row.push(row_values[j]);
        }
        grid.push(row);
    }

    let sol = Solution {};
    println!("{}", sol.max_score(grid));

    Ok(())
}