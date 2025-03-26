use std::io;
use std::cmp::max;

fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

fn rotate_grid(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

fn solve(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    
    for i in 0..m {
        let mut j = 0;
        dp[i][j] = 1;
        let mut x = i as i32 - 1;
        let mut y = j as i32 + 1;
        while x >= 0 && y < n as i32 {
            let i_prev = x as usize;
            let j_prev = y as usize;
            if suc(grid[i_prev][j_prev], grid[i_prev + 1][j_prev - 1]) {
                dp[i_prev][j_prev] = dp[i_prev + 1][j_prev - 1] + 1;
            } else {
                dp[i_prev][j_prev] = 1;
            }
            x -= 1;
            y += 1;
        }
    }

    for j in 1..n {
        let mut i = m - 1;
        dp[i][j] = 1;
        let mut x = i as i32 - 1;
        let mut y = j as i32 + 1;
        while x >= 0 && y < n as i32 {
            let i_prev = x as usize;
            let j_prev = y as usize;
            if suc(grid[i_prev][j_prev], grid[i_prev + 1][j_prev - 1]) {
                dp[i_prev][j_prev] = dp[i_prev + 1][j_prev - 1] + 1;
            } else {
                dp[i_prev][j_prev] = 1;
            }
            x -= 1;
            y += 1;
        }
    }

    let mut max_len = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                let mut current_len = 1;
                max_len = max(max_len, current_len);
                let mut x = i + 1;
                let mut y = j + 1;
                while x < m && y < n {
                    if current_len == 1 && grid[x][y] != 2 {
                        break;
                    }
                    if current_len > 1 && !suc(grid[x][y], grid[x - 1][y - 1]) {
                        break;
                    }
                    max_len = max(max_len, current_len + dp[x][y]);
                    current_len += 1;
                    x += 1;
                    y += 1;
                }
            }
        }
    }
    max_len
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut dims = input.trim().split_whitespace();
    let n: usize = dims.next().unwrap().parse().unwrap();
    let m: usize = dims.next().unwrap().parse().unwrap();
    
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = String::new();
        io::stdin().read_line(&mut row).unwrap();
        let row: Vec<i32> = row.trim()
                              .split_whitespace()
                              .map(|x| x.parse().unwrap())
                              .collect();
        grid.push(row);
    }
    
    let rotated1 = rotate_grid(&grid);
    let rotated2 = rotate_grid(&rotated1);
    let rotated3 = rotate_grid(&rotated2);
    
    let res1 = solve(&grid);
    let res2 = solve(&rotated1);
    let res3 = solve(&rotated2);
    let res4 = solve(&rotated3);
    
    let max_result = max(max(res1, res2), max(res3, res4));
    println!("{}", max_result);
}