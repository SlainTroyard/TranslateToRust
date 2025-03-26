use std::cmp;
use std::io::{self, BufRead};

// Global DP array for dynamic programming
static mut DP: [[i32; 500]; 500] = [[0; 500]; 500];

// Check if a succeeds b in the pattern
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

// Update the DP array for a diagonal starting at position (i, j)
fn update_dp(g: &Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    unsafe {
        DP[i][j] = 1;
        let mut i_curr = i as i32 - 1;
        let mut j_curr = j as i32 + 1;
        
        while i_curr >= 0 && j_curr < n as i32 {
            let i_curr_usize = i_curr as usize;
            let j_curr_usize = j_curr as usize;
            
            if suc(g[i_curr_usize][j_curr_usize], g[i_curr_usize + 1][j_curr_usize - 1]) {
                DP[i_curr_usize][j_curr_usize] = DP[i_curr_usize + 1][j_curr_usize - 1] + 1;
            } else {
                DP[i_curr_usize][j_curr_usize] = 1;
            }
            
            i_curr -= 1;
            j_curr += 1;
        }
    }
}

// Solve the problem for a given grid orientation
fn solve(g: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    // Update DP for all diagonals starting from left column
    for i in 0..m {
        update_dp(g, m, n, i, 0);
    }
    
    // Update DP for all diagonals starting from bottom row
    for j in 1..n {
        update_dp(g, m, n, m - 1, j);
    }
    
    let mut ans = 0;
    
    // Check all possible starting positions
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
                    
                    unsafe {
                        ans = cmp::max(len + DP[ii][jj], ans);
                    }
                    
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }
    
    ans
}

// Rotate the grid 90 degrees clockwise
fn rotate(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    
    let mut result = vec![vec![0; m]; n];
    
    for i in 0..n {
        for j in 0..m {
            result[i][j] = grid[j][n - 1 - i];
        }
    }
    
    result
}

// Main function to find the length of the V-diagonal
fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    
    // Rotate the grid three times to check all orientations
    let arr_1 = rotate(grid);
    let arr_2 = rotate(&arr_1);
    let arr_3 = rotate(&arr_2);
    
    // Solve for each orientation
    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);
    
    // Return the maximum result
    cmp::max(cmp::max(cmp::max(res_1, res_2), res_3), res_4)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read dimensions
    let dimensions = lines.next().unwrap().unwrap();
    let mut iter = dimensions.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = values[j];
        }
    }
    
    // Solve and print result
    println!("{}", len_of_v_diagonal(&grid));
}