use std::io;

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn rotate(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut rotated = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            rotated[i][j] = grid[j][n - 1 - i];
        }
    }
    rotated
}

fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![1; n]; m];

    // First pass: process each row starting from column 0
    for i in 0..m {
        let mut i_step = i - 1;
        let mut j_step = 1;
        while i_step >= 0 && j_step < n {
            let a = grid[i_step][j_step];
            let b = grid[i_step + 1][j_step - 1];
            if suc(a, b) {
                dp[i_step][j_step] = dp[i_step + 1][j_step - 1] + 1;
            } else {
                dp[i_step][j_step] = 1;
            }
            i_step -= 1;
            j_step += 1;
        }
    }

    // Second pass: process each column starting from the last row
    for j in 1..n {
        let mut i_step = m - 2;
        let mut j_step = j + 1;
        while i_step >= 0 && j_step < n {
            let a = grid[i_step][j_step];
            let b = grid[i_step + 1][j_step - 1];
            if suc(a, b) {
                dp[i_step][j_step] = dp[i_step + 1][j_step - 1] + 1;
            } else {
                dp[i_step][j_step] = 1;
            }
            i_step -= 1;
            j_step += 1;
        }
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = ans.max(1);
                let (mut ii, mut jj) = (i + 1, j + 1);
                let mut len = 1;
                loop {
                    if ii >= m || jj >= n {
                        break;
                    }
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

fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let rotated1 = rotate(grid);
    let rotated2 = rotate(&rotated1);
    let rotated3 = rotate(&rotated2);

    let res1 = solve(grid, m, n);
    let res2 = solve(&rotated1, rotated1.len(), rotated1[0].len());
    let res3 = solve(&rotated2, rotated2.len(), rotated2[0].len());
    let res4 = solve(&rotated3, rotated3.len(), rotated3[0].len());

    *[res1, res2, res3, res4].iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::new();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read row");
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        grid.push(row);
    }

    let result = len_of_v_diagonal(&grid);
    println!("{}", result);
}