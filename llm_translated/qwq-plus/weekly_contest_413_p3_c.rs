use std::io;

fn main() {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all tokens from stdin
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        tokens.extend(parts);
    }

    // Parse input
    let m = tokens[0] as usize;
    let grid_col_size: Vec<usize> = tokens[1..=m]
        .iter()
        .map(|&x| x as usize)
        .collect();
    let n = grid_col_size[0]; // Assume all rows have the same column count as the first row
    let total_elements = m * n;
    let expected_tokens = 1 + m + total_elements;
    assert!(tokens.len() >= expected_tokens);

    // Build the grid
    let mut grid = Vec::with_capacity(m);
    let mut ptr = m + 1;
    for i in 0..m {
        let row_start = ptr;
        ptr += n;
        let row = &tokens[row_start..ptr];
        grid.push(row.to_vec());
    }

    // Find the maximum number in the grid
    let mut maxnum = 0;
    for row in &grid {
        for &num in row {
            if num as usize > maxnum {
                maxnum = num as usize;
            }
        }
    }

    // Prepare nums_raw: bitmask for rows containing each number
    let m_rows = m;
    let maxnum = maxnum as usize;
    let mut nums_raw = vec![0u32; maxnum + 1];
    for i in 0..m {
        for &num in &grid[i] {
            let num_usize = num as usize;
            nums_raw[num_usize] |= 1 << i;
        }
    }

    // Initialize DP table
    let mut dp = vec![vec![i32::MIN; 1 << m]; maxnum + 1];
    dp[0][0] = 0;
    let mut ans = 0;

    // Compute DP transitions
    for i in 1..=maxnum {
        for j in 0..(1 << m) {
            // Option 1: don't take any new number for this step
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);

            // Option 2: consider taking a number from row k
            for k in 0..m {
                if (nums_raw[i] & (1 << k)) != 0 && (j & (1 << k)) != 0 {
                    let prev_j = j ^ (1 << k);
                    if dp[i - 1][prev_j] != i32::MIN {
                        let candidate = dp[i - 1][prev_j] + i as i32;
                        if candidate > dp[i][j] {
                            dp[i][j] = candidate;
                            if candidate > ans {
                                ans = candidate;
                            }
                        }
                    }
                }
            }
        }
    }

    // Output the result
    println!("{}", ans);
}