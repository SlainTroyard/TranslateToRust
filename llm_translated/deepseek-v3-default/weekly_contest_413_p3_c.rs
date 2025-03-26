use std::cmp::{max, min};
use std::io::{self, BufRead};

fn max_score(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: &Vec<usize>) -> i32 {
    let mut ans = 0;
    let mut max_num = 0;
    let m = grid_size;
    let n = grid_col_size[0];

    // Find the maximum number in the grid
    for i in 0..m {
        for j in 0..n {
            max_num = max(max_num, grid[i][j]);
        }
    }

    // Initialize nums_raw with size max_num + 1
    let mut nums_raw = vec![0; (max_num + 1) as usize];

    // Populate nums_raw with bitmasked row indices
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    // Initialize dp with size (max_num + 1) x (1 << (m + 1))
    let mut dp = vec![vec![i32::MIN; 1 << (m + 1)]; (max_num + 1) as usize];
    dp[0][0] = 0;

    // Dynamic programming to compute the maximum score
    for i in 1..=max_num as usize {
        for j in 0..(1 << m) {
            dp[i][j] = max(dp[i][j], dp[i - 1][j]);
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

    // Read grid size
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read grid column sizes
    let grid_col_size: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read grid values
    let mut grid = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Compute and print the maximum score
    let ans = max_score(&grid, grid_size, &grid_col_size);
    println!("{}", ans);
}