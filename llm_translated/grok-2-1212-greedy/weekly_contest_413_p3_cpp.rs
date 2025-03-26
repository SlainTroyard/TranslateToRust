use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_score(grid: Vec<Vec<i32>>) -> i32 {
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
            i: i32,
            j: i32,
        ) -> i32 {
            if i < 0 {
                return 0;
            }
            let i = i as usize;
            let j = j as usize;
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            memo[i][j] = dfs(all_nums, pos, memo, i as i32 - 1, j as i32);
            let x = all_nums[i];
            let mut t = *pos.get(&x).unwrap();
            while t != 0 {
                let lb = t & -t;
                if (j & lb as usize) == 0 {
                    memo[i][j] = memo[i][j].max(
                        dfs(all_nums, pos, memo, i as i32 - 1, (j | lb as usize) as i32) + x,
                    );
                }
                t ^= lb;
            }
            memo[i][j]
        }

        dfs(&all_nums, &pos, &mut memo, n as i32 - 1, 0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size
    let grid_size: usize = lines.next().unwrap()?.parse().unwrap();

    // Initialize grid
    let mut grid = Vec::with_capacity(grid_size);

    for _ in 0..grid_size {
        // Read column size for each row
        let grid_col_size: usize = lines.next().unwrap()?.parse().unwrap();

        // Read values for each row
        let row: Vec<i32> = lines
            .by_ref()
            .take(grid_col_size)
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        grid.push(row);
    }

    // Calculate and print the result
    let result = Solution::max_score(grid);
    println!("{}", result);

    Ok(())
}