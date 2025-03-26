use std::io::{self, BufRead};

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(grid: &[Vec<i32>], m: usize, n: usize, i: usize, j: usize, dp: &mut [Vec<i32>]) {
    dp[i][j] = 1;

    let mut current_i = i as isize - 1;
    let mut current_j = j + 1;

    while current_i >= 0 && current_j < n {
        let ui = current_i as usize;
        let uj = current_j;

        if suc(grid[ui][uj], grid[ui + 1][uj - 1]) {
            dp[ui][uj] = dp[ui + 1][uj - 1] + 1;
        } else {
            dp[ui][uj] = 1;
        }

        current_i -= 1;
        current_j += 1;
    }
}

fn solve(grid: &[Vec<i32>], m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        update_dp(grid, m, n, i, 0, &mut dp);
    }

    for j in 1..n {
        update_dp(grid, m, n, m - 1, j, &mut dp);
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

                    ans = ans.max(len as i32 + dp[ii][jj]);
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }

    ans
}

fn rotate(grid: &[Vec<i32>], m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

fn len_of_vdiagonal(grid: &[Vec<i32>], m: usize, n: usize) -> i32 {
    let arr_1 = rotate(grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    res_1.max(res_2).max(res_3).max(res_4)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid dimensions (rows and columns)
    let first_line = lines.next().unwrap().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();

    // Read grid data
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), cols, "Each row must have exactly {} elements", cols);
        grid.push(row);
    }

    let result = len_of_vdiagonal(&grid, rows, cols);
    println!("{}", result);
}