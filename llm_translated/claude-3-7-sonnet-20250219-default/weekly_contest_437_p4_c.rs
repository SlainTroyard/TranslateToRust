use std::cmp;
use std::io::{self, BufRead};

// Global DP array to memoize results
static mut DP: [[i32; 500]; 500] = [[0; 500]; 500];

/// Checks if a and b form a valid sequence (0 -> 2 or 2 -> 0)
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

/// Updates the DP array for a diagonal starting at position (i, j)
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

/// Main solver function, finds the longest valid diagonal
fn solve(g: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    // Update DP for all starting positions along the left edge
    for i in 0..m {
        update_dp(g, m, n, i, 0);
    }
    
    // Update DP for all starting positions along the bottom edge
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

/// Rotate the grid 90 degrees clockwise
fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; m]; n];
    
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    
    arr
}

/// Main function to compute the length of the valid diagonal
fn len_of_v_diagonal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: &[usize]) -> i32 {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m from the first line
    let first_line = lines.next().unwrap()?;
    let mut values = first_line.split_whitespace();
    let n: usize = values.next().unwrap().parse().unwrap();
    let m: usize = values.next().unwrap().parse().unwrap();
    
    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = values[j];
        }
    }
    
    let grid_col_size = [m];
    
    // Calculate and print the result
    println!("{}", len_of_v_diagonal(&grid, n, &grid_col_size));
    
    Ok(())
}