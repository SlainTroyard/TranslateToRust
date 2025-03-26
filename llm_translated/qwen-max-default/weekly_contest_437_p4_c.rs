use std::io::{self, BufRead, Write};

fn main() {
    // Read the grid dimensions from stdin
    let (n, m) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        (nums[0], nums[1])
    };

    // Read the grid values from stdin
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        grid[i] = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    // Calculate and print the result
    let result = len_of_v_diagonal(&grid);
    println!("{}", result);
}

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(grid: &Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) {
    dp[i][j] = 1;
    let mut i = i as isize - 1;
    let mut j = j as isize + 1;
    while i >= 0 && (j as usize) < n {
        if suc(grid[i as usize][j as usize], grid[(i + 1) as usize][(j - 1) as usize]) {
            dp[i as usize][j as usize] = dp[(i + 1) as usize][(j - 1) as usize] + 1;
        } else {
            dp[i as usize][j as usize] = 1;
        }
        i -= 1;
        j += 1;
    }
}

fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
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

    let arr_1 = rotate(&grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    let res_1 = solve(&grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    res_1.max(res_2).max(res_3).max(res_4)
}