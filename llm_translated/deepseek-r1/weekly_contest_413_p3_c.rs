use std::io::{self, Read};

fn max_score(grid: &[Vec<i32>], grid_size: usize, grid_col_size: &[usize]) -> i32 {
    let m = grid_size;
    let n = grid_col_size[0]; // Assuming all rows have at least 'n' columns as per original code

    // Find the maximum value in the grid
    let maxnum = *grid.iter().flatten().max().unwrap_or(&0);
    let maxnum = maxnum as usize;

    // nums_raw[val] contains bits representing rows that have the value 'val'
    let mut nums_raw = vec![0; maxnum + 1];
    for i in 0..m {
        for j in 0..n {
            let val = grid[i][j] as usize;
            nums_raw[val] |= 1 << i;
        }
    }

    let mask_size = 1 << m;
    // Initialize DP table with i32::MIN, except dp[0][0] = 0
    let mut dp = vec![vec![i32::MIN; mask_size]; maxnum + 1];
    dp[0][0] = 0;

    let mut ans = 0;

    for i in 1..=maxnum {
        for j in 0..mask_size {
            // Carry forward the previous maximum
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);

            // Try to select each row for current value
            for k in 0..m {
                let row_bit = 1 << k;
                // Check if current row has the value and is selected in the mask
                if (nums_raw[i] & row_bit) != 0 && (j & row_bit) != 0 {
                    let prev_mask = j ^ row_bit;
                    // Calculate new value safely to avoid overflow
                    let new_val = dp[i - 1][prev_mask].saturating_add(i as i32);
                    dp[i][j] = dp[i][j].max(new_val);
                    ans = ans.max(dp[i][j]);
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read grid size
    let grid_size = tokens.next().unwrap().parse::<usize>().unwrap();

    // Read column sizes for each row
    let mut grid_col_size = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        grid_col_size.push(tokens.next().unwrap().parse::<usize>().unwrap());
    }

    // Read grid values, each row has its specified column count
    let mut grid = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        let cols = grid_col_size[i];
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(tokens.next().unwrap().parse::<i32>().unwrap());
        }
        grid.push(row);
    }

    let ans = max_score(&grid, grid_size, &grid_col_size);
    println!("{}", ans);
}