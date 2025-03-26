fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn update_dp(g: &Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) {
    dp[i][j] = 1;
    let mut current_i = i as i32 - 1;
    let mut current_j = j + 1;
    while current_i >= 0 && current_j < n as i32 {
        let current_i_usize = current_i as usize;
        let current_j_usize = current_j as usize;
        if suc(g[current_i_usize][current_j_usize], g[current_i_usize + 1][current_j_usize - 1]) {
            dp[current_i_usize][current_j_usize] = dp[current_i_usize + 1][current_j_usize - 1] + 1;
        } else {
            dp[current_i_usize][current_j_usize] = 1;
        }
        current_i -= 1;
        current_j += 1;
    }
}

fn solve(g: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

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

fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut arr: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

fn len_of_v_diagonal(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        let mut row_input = String::new();
        std::io::stdin().read_line(&mut row_input).unwrap();
        let row_iter = row_input.trim().split_whitespace();
        let row_vec: Vec<i32> = row_iter
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..m {
            grid[i][j] = row_vec[j];
        }
    }

    println!("{}", len_of_v_diagonal(&grid, n, m));
}