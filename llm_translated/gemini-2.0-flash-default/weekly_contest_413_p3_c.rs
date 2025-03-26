use std::cmp::max;
use std::io;
use std::io::Read;

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut max_num = 0;
    for i in 0..m {
        for j in 0..n {
            max_num = max(max_num, grid[i][j]);
        }
    }

    let mut nums_raw: Vec<i32> = vec![0; (max_num + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MIN; 1 << m]; (max_num + 1) as usize];
    dp[0][0] = 0;

    let mut ans = 0;
    for i in 1..=max_num {
        for j in 0..(1 << m) {
            dp[i as usize][j] = max(dp[i as usize][j], dp[(i - 1) as usize][j]);
            //选第k行元素
            for k in 0..m {
                if (nums_raw[i as usize] >> k & 1) == 1 && (j >> k & 1) == 1 {
                    dp[i as usize][j] = max(
                        dp[i as usize][j],
                        dp[(i - 1) as usize][j ^ (1 << k)] + i,
                    );
                    ans = max(ans, dp[i as usize][j]);
                }
            }
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let grid_size: usize = lines.next().unwrap().parse().unwrap();

    let mut grid_col_size: Vec<usize> = Vec::new();
    let col_size_line = lines.next().unwrap();
    for s in col_size_line.split_whitespace() {
        grid_col_size.push(s.parse().unwrap());
    }

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..grid_size {
        let row_line = lines.next().unwrap();
        let mut row: Vec<i32> = Vec::new();
        for s in row_line.split_whitespace() {
            row.push(s.parse().unwrap());
        }
        grid.push(row);
    }

    let ans = max_score(&grid);
    println!("{}", ans);

    Ok(())
}