use std::cmp::max;
use std::io;

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut maxnum = 0;
    let m = grid.len();
    let n = if m > 0 { grid[0].len() } else { 0 };

    // Find the maximum number in the grid
    for i in 0..m {
        for j in 0..n {
            maxnum = max(maxnum, grid[i][j]);
        }
    }

    // numsRaw[x] will store a bitmask indicating rows where number x appears
    let mut nums_raw: Vec<i32> = vec![0; (maxnum + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    // dp[i][j] stores the maximum score achievable using numbers up to i with rows represented by bitmask j being selected
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MIN; 1 << m]; (maxnum + 1) as usize];
    dp[0][0] = 0;

    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            dp[i as usize][j] = max(dp[i as usize][j], dp[(i - 1) as usize][j]); // Option to not use number i
            // Try to select number i in each row k
            for k in 0..m {
                // Check if number i is present in row k AND if row k is selected in bitmask j
                if (nums_raw[i as usize] >> k) & 1 != 0 && (j >> k) & 1 != 0 {
                    dp[i as usize][j] = max(
                        dp[i as usize][j],
                        dp[(i - 1) as usize][j ^ (1 << k)] + i, // Select number i in row k
                    );
                    ans = max(ans, dp[i as usize][j]); // Update overall maximum answer
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

    let mut gridColSize_line = String::new();
    io::stdin().read_line(&mut gridColSize_line).unwrap();
    let gridColSize_vec: Vec<usize> = gridColSize_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for i in 0..gridSize {
        let mut row_line = String::new();
        io::stdin().read_line(&mut row_line).unwrap();
        let row_vec: Vec<i32> = row_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row_vec);
    }

    let ans = max_score(&grid);
    println!("{}", ans);
}