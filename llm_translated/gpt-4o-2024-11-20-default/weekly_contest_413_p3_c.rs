use std::cmp::{max, min};
use std::io::{self, BufRead};
use std::usize;

fn max_score(grid: &[Vec<i32>]) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    let mut maxnum = 0;

    // Find the maximum number in the grid
    for row in grid {
        for &value in row {
            maxnum = max(maxnum, value);
        }
    }

    // nums_raw array, where nums_raw[i] represents the rows (bitmask) containing value `i`
    let mut nums_raw = vec![0; (maxnum + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    // dp table, where dp[i][j] represents the max score using values <= i with row mask j
    let mut dp = vec![vec![i32::MIN; 1 << m]; (maxnum + 1) as usize];
    dp[0][0] = 0;

    for i in 1..=maxnum as usize {
        for j in 0..(1 << m) {
            // Not choosing any row for value `i`
            dp[i][j] = max(dp[i][j], dp[i - 1][j]);

            // Choosing specific rows
            for k in 0..m {
                if (nums_raw[i] >> k & 1) != 0 && (j >> k & 1) != 0 {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j ^ (1 << k)] + i as i32);
                    ans = max(ans, dp[i][j]);
                }
            }
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read gridSize (number of rows)
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read gridColSize (number of columns per row)
    let mut grid_col_size = vec![0; grid_size];
    let mut grid = vec![Vec::new(); grid_size];

    for i in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        grid_col_size[i] = nums.clone().count();
        grid[i] = nums.collect();
    }

    let result = max_score(&grid);
    println!("{}", result);
}