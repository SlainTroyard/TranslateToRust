use std::io::{self, BufRead};

// Global 2D array for dynamic programming
static mut DP: [[i32; 500]; 500] = [[0; 500]; 500];

// Function to check if a move is successful
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

// Function to update the DP array
fn update_dp(grid: &Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    unsafe {
        DP[i][j] = 1;
        let mut i = i;
        let mut j = j;
        while i > 0 && j < n - 1 {
            i -= 1;
            j += 1;
            if suc(grid[i][j], grid[i + 1][j - 1]) {
                DP[i][j] = DP[i + 1][j - 1] + 1;
            } else {
                DP[i][j] = 1;
            }
        }
    }
}

// Main solving function
fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    for i in 0..m {
        update_dp(grid, m, n, i, 0);
    }
    for j in 1..n {
        update_dp(grid, m, n, m - 1, j);
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = ans.max(1);
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
                    unsafe {
                        ans = ans.max(len + DP[ii][jj]);
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

// Function to rotate the grid
fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut rotated = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            rotated[i][j] = grid[j][n - 1 - i];
        }
    }
    rotated
}

// Main function to find the length of the longest valid diagonal
fn len_of_v_diagonal(grid: &Vec<Vec<i32>>, grid_size: usize) -> i32 {
    let m = grid_size;
    let n = grid[0].len();

    let arr_1 = rotate(grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    res_1.max(res_2).max(res_3).max(res_4)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input dimensions
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read grid
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    // Calculate and print result
    let result = len_of_v_diagonal(&grid, n);
    println!("{}", result);

    Ok(())
}