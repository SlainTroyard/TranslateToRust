use std::io;
use std::vec::Vec;

fn maxScore(grid: &Vec<Vec<i32>>, gridSize: i32, gridColSize: &Vec<i32>) -> i32 {
    let m = gridSize as usize;
    let n = gridColSize[0] as usize;

    let mut maxnum = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] > maxnum {
                maxnum = grid[i][j];
            }
        }
    }

    let maxnum = maxnum as usize;
    let mut numsRaw = vec![0; maxnum + 1];
    for i in 0..m {
        for j in 0..n {
            let val = grid[i][j] as usize;
            numsRaw[val] |= 1 << i;
        }
    }

    let mut dp = vec![vec![i32::MIN; 1 << m]; maxnum + 1];
    dp[0][0] = 0;

    let mut ans = 0;
    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            dp[i][j] = dp[i-1][j];

            for k in 0..m {
                if (numsRaw[i] >> k) & 1 != 0 && (j >> k) & 1 != 0 {
                    let prev_j = j ^ (1 << k);
                    if dp[i-1][prev_j] + (i as i32) > dp[i][j] {
                        dp[i][j] = dp[i-1][prev_j] + (i as i32);
                    }
                    if dp[i][j] > ans {
                        ans = dp[i][j];
                    }
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input = io::read_to_string(io::stdin()).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut ptr = 0;
    let gridSize = tokens[ptr].parse::<i32>().unwrap();
    ptr += 1;

    let mut gridColSize = Vec::new();
    for _ in 0..gridSize {
        gridColSize.push(tokens[ptr].parse::<i32>().unwrap());
        ptr += 1;
    }

    let mut grid = Vec::new();
    for i in 0..gridSize {
        let row_size = gridColSize[i as usize];
        let mut row = Vec::new();
        for _ in 0..row_size {
            row.push(tokens[ptr].parse::<i32>().unwrap());
            ptr += 1;
        }
        grid.push(row);
    }

    let ans = maxScore(&grid, gridSize, &gridColSize);
    println!("{}", ans);
}