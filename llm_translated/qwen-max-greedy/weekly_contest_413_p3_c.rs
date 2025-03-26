use std::io::{self, BufRead, Write};

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut maxnum = 0;
    for row in grid.iter() {
        for &val in row.iter() {
            maxnum = maxnum.max(val);
        }
    }

    let mut nums_raw = vec![0; maxnum as usize + 1];
    for (i, row) in grid.iter().enumerate() {
        for &val in row.iter() {
            nums_raw[val as usize] |= 1 << i;
        }
    }

    let mut dp = vec![vec![i32::MIN; 1 << m]; maxnum as usize + 1];
    dp[0][0] = 0;

    for i in 1..=maxnum as usize {
        for j in 0..(1 << m) {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            for k in 0..m {
                if (nums_raw[i] >> k & 1) != 0 && (j >> k & 1) != 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j ^ (1 << k)] + i as i32);
                }
            }
        }
    }

    *dp[maxnum as usize].iter().max().unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut lines = stdin.lock().lines();

    // Read the number of rows
    let gridSize: usize = lines.next().unwrap()?.parse().unwrap();

    // Read the number of columns for each row
    let mut gridColSize = Vec::with_capacity(gridSize);
    for _ in 0..gridSize {
        gridColSize.push(lines.next().unwrap()?.parse().unwrap());
    }

    // Read the grid values
    let mut grid = Vec::with_capacity(gridSize);
    for _ in 0..gridSize {
        let row: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Calculate the maximum score
    let ans = max_score(&grid);

    // Output the result
    writeln!(stdout, "{}", ans)?;

    Ok(())
}