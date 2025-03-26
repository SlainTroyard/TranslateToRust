use std::io;

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

    dp[maxnum as usize].iter().max().unwrap().clone()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let grid_size: usize = input.trim().parse().expect("Please type a number!");

    let mut grid_col_sizes = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let col_size: usize = input.trim().parse().expect("Please type a number!");
        grid_col_sizes.push(col_size);
    }

    let mut grid = vec![vec![0; grid_col_sizes[0]]; grid_size];
    for i in 0..grid_size {
        for j in 0..grid_col_sizes[i] {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            grid[i][j] = input.trim().parse().expect("Please type a number!");
        }
    }

    let ans = max_score(&grid);
    println!("{}", ans);
}