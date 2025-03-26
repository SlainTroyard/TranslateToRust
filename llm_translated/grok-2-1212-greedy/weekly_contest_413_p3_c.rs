use std::io::{self, BufRead};
use std::cmp::{max, min};

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut maxnum = 0;
    for row in grid.iter() {
        for &num in row.iter() {
            maxnum = max(maxnum, num);
        }
    }

    let mut nums_raw = vec![0; (maxnum + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    let mut dp = vec![vec![i32::MIN; 1 << m]; (maxnum + 1) as usize];
    dp[0][0] = 0;

    let mut ans = 0;
    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            dp[i as usize][j] = max(dp[i as usize][j], dp[(i - 1) as usize][j]);
            for k in 0..m {
                if (nums_raw[i as usize] >> k & 1) != 0 && (j >> k & 1) != 0 {
                    dp[i as usize][j] = max(dp[i as usize][j], dp[(i - 1) as usize][j ^ (1 << k)] + i);
                    ans = max(ans, dp[i as usize][j]);
                }
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size
    let grid_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read grid column sizes
    let mut grid_col_size = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        grid_col_size.push(lines.next().unwrap()?.trim().parse().unwrap());
    }

    // Read grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let row: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Calculate and print the result
    let ans = max_score(&grid);
    println!("{}", ans);

    Ok(())
}