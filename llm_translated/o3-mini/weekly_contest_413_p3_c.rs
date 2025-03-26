use std::cmp;
use std::error::Error;
use std::i32;
use std::io::{self, Read};

/// Compute the maximum score as described in the original C code.
/// This function implements dynamic programming over numbers and masks.
fn max_score(grid: &[Vec<i32>]) -> i32 {
    let m = grid.len(); // number of rows
    let n = grid[0].len(); // number of columns (assumed uniform)
    let mut ans = 0;
    let mut maxnum = 0;

    // 1. Find the maximum number in the grid.
    for row in grid {
        for &num in row {
            maxnum = cmp::max(maxnum, num);
        }
    }

    // 2. Build an array to store, for each number from 0..=maxnum,
    // a bitmask indicating in which rows this number appears.
    // Use u32 for the mask since m is small.
    let size = (maxnum + 1) as usize;
    let mut nums_raw = vec![0u32; size];
    for i in 0..m {
        for &num in &grid[i] {
            // Set the bit for row i if this number appears in that row.
            nums_raw[num as usize] |= 1 << i;
        }
    }

    // 3. Prepare DP table:
    // dp[i][mask] = maximum score after processing numbers 0..i (inclusive of i's effect)
    // where mask is a bitmask of size m.
    // There are (maxnum+1) rows and (1 << m) possible masks.
    let mask_size = 1 << m;
    let mut dp = vec![vec![i32::MIN; mask_size]; (maxnum as usize) + 1];
    dp[0][0] = 0;

    // 4. DP transitions for each number 1..=maxnum.
    // We iterate over all numbers and all masks.
    for i in 1..=maxnum as usize {
        for mask in 0..mask_size {
            // Propagate the previous state: not using the current number.
            dp[i][mask] = cmp::max(dp[i][mask], dp[i - 1][mask]);
            // Try to select row k, if:
            // - The current number i appears in row k.
            // - The mask has bit k set.
            // Then update the dp state for the current number.
            for k in 0..m {
                if (nums_raw[i] >> k) & 1 == 1 && (mask >> k) & 1 == 1 {
                    // Turn off bit k in the mask.
                    let prev_mask = mask ^ (1 << k);
                    let candidate = dp[i - 1][prev_mask];
                    // Only update if the previous state is not negative infinity.
                    if candidate != i32::MIN {
                        let candidate = candidate + (i as i32);
                        dp[i][mask] = cmp::max(dp[i][mask], candidate);
                        ans = cmp::max(ans, dp[i][mask]);
                    }
                }
            }
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Tokenize input (split by whitespace) to mimic C scanf behavior.
    let mut tokens = input.split_whitespace();

    // Read gridSize (number of rows)
    let grid_size: usize = tokens
        .next()
        .ok_or("Missing grid size")?
        .parse()
        .map_err(|_| "Failed parsing grid size")?;

    // Read the gridColSize array.
    // According to the original C code, there are grid_size integers.
    let mut grid_col_sizes = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let size: usize = tokens
            .next()
            .ok_or("Missing grid column size")?
            .parse()
            .map_err(|_| "Failed parsing grid column size")?;
        grid_col_sizes.push(size);
    }

    // Build the grid.
    // For each row, read grid_col_sizes[i] numbers.
    let mut grid = Vec::with_capacity(grid_size);
    for &cols in &grid_col_sizes {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            let num: i32 = tokens
                .next()
                .ok_or("Missing grid element")?
                .parse()
                .map_err(|_| "Failed parsing grid element")?;
            row.push(num);
        }
        grid.push(row);
    }

    // Compute the answer using the max_score function.
    let ans = max_score(&grid);

    // Print the answer following the output format.
    println!("{}", ans);

    Ok(())
}