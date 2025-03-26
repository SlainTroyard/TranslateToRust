use std::cmp::max;
use std::io;

const MAX_SIZE: usize = 500;

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(g: &Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) {
    dp[i][j] = 1;
    let mut cur_i = i as i32 - 1;
    let mut cur_j = j + 1;

    while cur_i >= 0 && cur_j < n {
        let cur_i_usize = cur_i as usize;
        if suc(g[cur_i_usize][cur_j], g[cur_i_usize + 1][cur_j - 1]) {
            dp[cur_i_usize][cur_j] = dp[cur_i_usize + 1][cur_j - 1] + 1;
        } else {
            dp[cur_i_usize][cur_j] = 1;
        }
        cur_i -= 1;
        cur_j += 1;
    }
}

fn solve(g: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![0; MAX_SIZE]; MAX_SIZE];

    for i in 0..m {
        update_dp(g, m, n, i, 0, &mut dp);
    }

    for j in 1..n {
        update_dp(g, m, n, m - 1, j, &mut dp);
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

                    ans = max(len + dp[ii][jj], ans);

                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }

    ans as i32
}

fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
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

    let arr_1 = rotate(grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    max(max(max(res_1, res_2), res_3), res_4)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut grid = Vec::new();
    for _ in 0..n {
        let mut row = Vec::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let iter = input.trim().split_whitespace();
        for s in iter {
            row.push(s.parse::<i32>().unwrap());
        }
        grid.push(row);
    }

    println!("{}", len_of_v_diagonal(&grid));
}