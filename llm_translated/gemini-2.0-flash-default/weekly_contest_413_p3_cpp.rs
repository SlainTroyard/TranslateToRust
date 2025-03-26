use std::collections::HashMap;
use std::io;
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

            if memo[i - 1][j as usize] != -1 {
                return memo[i - 1][j as usize];
            }

            let mut res = dfs(i - 1, j, all_nums, pos, memo);

            let x = all_nums[i - 1];
            let t = pos.get(&x).unwrap();

            let mut temp_t = *t;
            while temp_t > 0 {
                let lb = temp_t & -temp_t;
                if (j & lb) == 0 {
                    res = max(res, dfs(i - 1, j | lb, all_nums, pos, memo) + x);
                }
                temp_t ^= lb;
            }

            memo[i - 1][j as usize] = res;
            res
        }

        dfs(n, 0, &all_nums, &pos, &mut memo)
    }
}

fn main() {
    let mut grid_size_str = String::new();
    io::stdin().read_line(&mut grid_size_str).unwrap();
    let grid_size: usize = grid_size_str.trim().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..grid_size {
        let mut grid_col_size_str = String::new();
        io::stdin().read_line(&mut grid_col_size_str).unwrap();
        let grid_col_size: usize = grid_col_size_str.trim().parse().unwrap();

        let mut row_str = String::new();
        io::stdin().read_line(&mut row_str).unwrap();
        let row: Vec<i32> = row_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        assert_eq!(row.len(), grid_col_size);
        grid.push(row);
    }

    let sol = Solution {};
    println!("{}", sol.max_score(grid));
}