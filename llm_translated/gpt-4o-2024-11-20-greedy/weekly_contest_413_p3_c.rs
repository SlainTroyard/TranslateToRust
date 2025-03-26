use std::cmp::max;
use std::io::{self, BufRead};

fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut maxnum = 0;

    // Find the maximum number in the grid
    for i in 0..m {
        for j in 0..n {
            maxnum = max(maxnum, grid[i][j]);
        }
    }

    // Create nums_raw array
    let mut nums_raw = vec![0; (maxnum + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    // Create dp array
    let mut dp = vec![vec![i32::MIN; 1 << m]; (maxnum + 1) as usize];
    dp[0][0] = 0;

    let mut ans = 0;

    // Dynamic programming to calculate the maximum score
    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            dp[i as usize][j] = max(dp[i as usize][j], dp[(i - 1) as usize][j]);
            for k in 0..m {
                if (nums_raw[i as usize] >> k & 1) != 0 && (j >> k & 1) != 0 {
                    dp[i as usize][j] = max(
                        dp[i as usize][j],
                        dp[(i - 1) as usize][j ^ (1 << k)] + i,
                    );
                    ans = max(ans, dp[i as usize][j]);
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
    let grid_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse grid size");

    // Read grid column sizes
    let grid_col_sizes: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse grid column size"))
        .collect();

    // Read the grid
    let mut grid = Vec::new();
    for i in 0..grid_size {
        let row: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse grid value"))
            .collect();
        assert_eq!(row.len(), grid_col_sizes[i], "Row size mismatch");
        grid.push(row);
    }

    // Calculate the maximum score
    let ans = max_score(grid);

    // Print the result
    println!("{}", ans);
}