use std::io::{self, BufRead, Write};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line for grid dimensions
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse().unwrap();
        }
    }

    // Call the main function and print the result
    let result = len_of_v_diagonal(&grid);
    println!("{}", result);
}

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(g: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) {
    dp[i][j] = 1;
    let mut i = i - 1;
    let mut j = j + 1;
    while i < m && j < n {
        if suc(g[i][j], g[i + 1][j - 1]) {
            dp[i][j] = dp[i + 1][j - 1] + 1;
        } else {
            dp[i][j] = 1;
        }
        if i > 0 { i -= 1; }
        if j < n - 1 { j += 1; }
    }
}

fn solve(g: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        update_dp(g, &mut dp, m, n, i, 0);
    }

    for j in 1..n {
        update_dp(g, &mut dp, m, n, m - 1, j);
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 1 {
                ans = ans.max(1);
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

                    ans = ans.max(len + dp[ii][jj]);
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }

    ans
}

fn rotate(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut arr = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }

    arr
}

fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let arr_1 = rotate(&grid);
    let arr_2 = rotate(&arr_1);
    let arr_3 = rotate(&arr_2);

    let res_1 = solve(&grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    res_1.max(res_2).max(res_3).max(res_4)
}