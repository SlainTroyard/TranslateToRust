use std::cmp::max;
use std::io::{self, BufRead};

const MAX_SIZE: usize = 500;
static mut DP: [[i32; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(grid: &[Vec<i32>], m: usize, n: usize, mut i: usize, mut j: usize) {
    unsafe {
        DP[i][j] = 1;
    }
    if i > 0 {
        i -= 1;
    }
    j += 1;
    while i < m && j < n {
        unsafe {
            if suc(grid[i][j], grid[i + 1][j - 1]) {
                DP[i][j] = DP[i + 1][j - 1] + 1;
            } else {
                DP[i][j] = 1;
            }
        }
        if i > 0 {
            i -= 1;
        }
        j += 1;
    }
}

fn solve(grid: &[Vec<i32>], m: usize, n: usize) -> i32 {
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

fn rotate(grid: &[Vec<i32>], m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

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
    let mut dimensions = first_line.split_whitespace();
    let n: usize = dimensions.next().unwrap().parse().unwrap();
    let m: usize = dimensions.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values = line.split_whitespace();
        for (j, value) in values.enumerate() {
            grid[i][j] = value.parse().unwrap();
        }
    }

    let result = len_of_v_diagonal(grid, n, m);
    println!("{}", result);
}