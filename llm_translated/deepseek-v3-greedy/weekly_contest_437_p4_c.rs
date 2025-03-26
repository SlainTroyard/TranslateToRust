use std::cmp::max;
use std::io::{self, BufRead};

// Function to check if two grid values are successful
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

// Function to update the DP table
fn update_dp(grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    dp[i][j] = 1;
    let mut i = i as i32 - 1;
    let mut j = j + 1;
    while i >= 0 && j < n {
        let i_usize = i as usize;
        if suc(grid[i_usize][j], grid[(i + 1) as usize][j - 1]) {
            dp[i_usize][j] = dp[(i + 1) as usize][j - 1] + 1;
        } else {
            dp[i_usize][j] = 1;
        }
        i -= 1;
        j += 1;
    }
}

// Function to solve the problem
fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        update_dp(grid, &mut dp, m, n, i, 0);
    }
    for j in 1..n {
        update_dp(grid, &mut dp, m, n, m - 1, j);
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = max(ans, 1);
                let mut ii = i + 1;
                let mut jj = j + 1;
                let mut len = 1;
                while ii < m && jj < n {
                    if len == 1 && grid[ii][jj] != 2 {
                        break;
                    }
                    if len > 1 && !suc(grid[ii][jj], grid[ii - 1][jj - 1]) {
                        break;
                    }
                    ans = max(ans, len + dp[ii][jj]);
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }
    ans
}

// Function to rotate the grid 90 degrees clockwise
fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

// Main function to solve the problem
fn len_of_v_diagonal(grid: Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> i32 {
    let m = grid_size;
    let n = grid_col_size;

    let arr_1 = rotate(&grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    let res_1 = solve(&grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    max(max(max(res_1, res_2), res_3), res_4)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    let grid_col_size = vec![m];
    let result = len_of_v_diagonal(grid, n, grid_col_size[0]);
    println!("{}", result);
}