use std::io::{self, Read, Write};
use std::cmp::max;

// Global 2D array for dynamic programming
static mut DP: [[i32; 500]; 500] = [[0; 500]; 500];

// Check if two values form a successful pair
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

// Update the DP table
fn update_dp(g: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    unsafe {
        DP[i][j] = 1;
        let mut i = i as i32 - 1;
        let mut j = j as i32 + 1;
        while i >= 0 && j < n as i32 {
            if suc(g[i as usize][j as usize], g[i as usize + 1][j as usize - 1]) {
                DP[i as usize][j as usize] = DP[i as usize + 1][j as usize - 1] + 1;
            } else {
                DP[i as usize][j as usize] = 1;
            }
            i -= 1;
            j += 1;
        }
    }
}

// Solve the problem for a given grid
fn solve(g: &mut Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    for i in 0..m {
        update_dp(g, m, n, i, 0);
    }
    for j in 1..n {
        update_dp(g, m, n, m - 1, j);
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 1 {
                ans = max(ans, 1);
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
                        ans = max(len + DP[ii][jj], ans);
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
fn len_of_v_diagonal(grid: &mut Vec<Vec<i32>>, grid_size: usize) -> i32 {
    let m = grid_size;
    let n = grid[0].len();

    let mut arr_1 = rotate(grid, m, n);
    let mut arr_2 = rotate(&mut arr_1, n, m);
    let mut arr_3 = rotate(&mut arr_2, m, n);

    let res_1 = solve(grid, m, n);
    let res_2 = solve(&mut arr_1, n, m);
    let res_3 = solve(&mut arr_2, m, n);
    let res_4 = solve(&mut arr_3, n, m);

    max(max(max(res_1, res_2), res_3), res_4)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let (n, m) = {
        let mut nm = first_line.split_whitespace();
        (nm.next().unwrap().parse::<usize>().unwrap(), nm.next().unwrap().parse::<usize>().unwrap())
    };

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    let result = len_of_v_diagonal(&mut grid, n);
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}