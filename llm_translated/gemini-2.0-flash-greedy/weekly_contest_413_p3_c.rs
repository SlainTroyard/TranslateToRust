use std::cmp::{max, min};
use std::io;

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut maxnum = 0;
    for i in 0..m {
        for j in 0..n {
            maxnum = max(maxnum, grid[i][j]);
        }
    }

    let mut nums_raw: Vec<i32> = vec![0; (maxnum + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MIN; 1 << m]; (maxnum + 1) as usize];
    dp[0][0] = 0;

    let mut ans = 0;
    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            dp[i as usize][j as usize] = max(dp[i as usize][j as usize], dp[(i - 1) as usize][j as usize]);
            //选第k行元素
            for k in 0..m {
                if (nums_raw[i as usize] >> k & 1) != 0 && (j >> k & 1) != 0 {
                    dp[i as usize][j as usize] = max(
                        dp[i as usize][j as usize],
                        dp[(i - 1) as usize][(j ^ (1 << k)) as usize] + i,
                    );
                    ans = max(ans, dp[i as usize][j as usize]);
                }
            }
        }
    }
    ans
}

fn main() {
    let mut gridSize_str = String::new();
    io::stdin().read_line(&mut gridSize_str).unwrap();
    let gridSize: usize = gridSize_str.trim().parse().unwrap();

    let mut gridColSize_str = String::new();
    io::stdin().read_line(&mut gridColSize_str).unwrap();
    let gridColSize: Vec<usize> = gridColSize_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for i in 0..gridSize {
        let mut row_str = String::new();
        io::stdin().read_line(&mut row_str).unwrap();
        let row: Vec<i32> = row_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let ans = max_score(&grid);
    println!("{}", ans);
}