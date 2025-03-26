use std::io::{self, BufRead};
use std::cmp;

/// Returns true if the pair is (0,2) or (2,0)
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

/// Updates the dynamic programming table for a specific starting point
fn update_dp(g: &[Vec<i32>], dp: &mut [Vec<i32>], n: usize, i: usize, j: usize) {
    dp[i][j] = 1;
    
    let mut i_curr = i as isize - 1;
    let mut j_curr = j as isize + 1;
    
    while i_curr >= 0 && j_curr < n as isize {
        let i_curr_usize = i_curr as usize;
        let j_curr_usize = j_curr as usize;
        
        if suc(g[i_curr_usize][j_curr_usize], g[i_curr_usize + 1][j_curr_usize - 1]) {
            dp[i_curr_usize][j_curr_usize] = dp[i_curr_usize + 1][j_curr_usize - 1] + 1;
        } else {
            dp[i_curr_usize][j_curr_usize] = 1;
        }
        
        i_curr -= 1;
        j_curr += 1;
    }
}

/// Solves the problem for a given grid orientation
fn solve(g: &[Vec<i32>], m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; n]; m];
    
    // Update dp for all cells in the leftmost column
    for i in 0..m {
        update_dp(g, &mut dp, n, i, 0);
    }
    
    // Update dp for all cells in the bottom row (except the first which was already done)
    for j in 1..n {
        update_dp(g, &mut dp, n, m - 1, j);
    }
    
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 1 {
                ans = cmp::max(ans, 1);
                let mut ii = i + 1;
                let mut jj = j + 1;
                let mut len = 1;
                
                while ii < m && jj < n {
                    if len == 1 && g[ii][jj] != 2 {
                        break;
                    }
                    
                    if len > 1 && !suc(g[ii][jj], g[ii - 1][jj - 1]) {
                        break;
                    }
                    
                    ans = cmp::max(len + dp[ii][jj], ans);
                    
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }
    
    ans
}

/// Rotates the grid 90 degrees clockwise
fn rotate(grid: &[Vec<i32>], m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

/// Calculates the length of V Diagonal
fn len_of_v_diagonal(grid: &[Vec<i32>], grid_size: usize, grid_col_size: &[usize]) -> i32 {
    let m = grid_size;
    let n = grid_col_size[0];
    
    let arr_1 = rotate(grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);
    
    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);
    
    cmp::max(cmp::max(cmp::max(res_1, res_2), res_3), res_4)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first line containing dimensions
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = dimensions[0]; // rows
    let m = dimensions[1]; // columns
    
    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = row[j];
        }
    }
    
    let grid_col_size = [m]; // Equivalent to gridColSize in C code
    
    // Output the result
    println!("{}", len_of_v_diagonal(&grid, n, &grid_col_size));
}