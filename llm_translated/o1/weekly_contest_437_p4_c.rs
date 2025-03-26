// LeetCode Weekly Contest 437 Problem 4 in Rust
// --------------------------------------------
// This program reads a 2D grid of integers from stdin,
// processes it according to the original C solution logic,
// and prints the result to stdout.
//
// The input format is exactly the same as the C code:
//   First line: two integers n m
//   Next n lines: each line has m integers
//
// The output is a single integer on a line, just like the C code.

use std::io::{self, BufRead};

// Helper function that checks if the pair (a, b) is a "successful" match
// in this problem's specific sense.
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

// Update the dp array along an anti-diagonal starting from (i_start, j_start).
// The dp array tracks how many successive "suc()" matches we can chain
// going upward-left to downward-right.
fn update_dp(
    grid: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
    m: usize,
    n: usize,
    mut i: i32,
    mut j: i32,
) {
    dp[i as usize][j as usize] = 1;

    i -= 1;
    j += 1;
    while i >= 0 && (j as usize) < n {
        let up_i = (i + 1) as usize;
        let up_j = (j - 1) as usize;
        if suc(grid[i as usize][j as usize], grid[up_i][up_j]) {
            dp[i as usize][j as usize] = dp[up_i][up_j] + 1;
        } else {
            dp[i as usize][j as usize] = 1;
        }
        i -= 1;
        j += 1;
    }
}

// Solve the core problem on a grid of size m x n, returning an integer score.
fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    // dp[i][j] = length of consecutive "suc()" matches when moving
    // along the upward-left -> downward-right diagonal ending at (i, j).
    let mut dp = vec![vec![0; n]; m];

    // First fill dp for each cell in the leftmost column
    for i in 0..m {
        update_dp(grid, &mut dp, m, n, i as i32, 0);
    }
    // Then fill dp for each cell in the bottom row (excluding the corner already done)
    for j in 1..n {
        update_dp(grid, &mut dp, m, n, (m - 1) as i32, j as i32);
    }

    let mut ans = 0;

    // Traverse each cell to see if we can form a longer diagonal using the dp array
    // and the special matching rules.
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                // We can always form a diagonal of length 1 if we see a '1'
                ans = ans.max(1);

                let mut ii = i + 1;
                let mut jj = j + 1;
                let mut len = 1;

                // Attempt to extend diagonally
                while ii < m && jj < n {
                    if len == 1 && grid[ii][jj] != 2 {
                        // Must see a '2' immediately after '1' if length is 1
                        break;
                    }
                    if len > 1
                        && !suc(grid[ii][jj], grid[ii - 1][jj - 1])
                    {
                        // Must continue matching rules if length > 1
                        break;
                    }
                    ans = ans.max(len + dp[ii][jj]);
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }

    ans
}

// Rotate the given grid (m x n) by 90 degrees clockwise.
// The returned grid is (n x m).
fn rotate(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    // After rotation, the new grid has dimensions n x m.
    let mut arr = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    arr
}

// Compute the length of the "vertical diagonal" defined by the problem.
// We rotate the grid up to 3 times to check all orientations, just like the C code.
fn len_of_v_diagonal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: &[usize]) -> i32 {
    let m = grid_size;
    let n = grid_col_size[0];

    // Perform the series of rotations
    let arr_1 = rotate(grid, m, n);
    let arr_2 = rotate(&arr_1, n, m);
    let arr_3 = rotate(&arr_2, m, n);

    // Solve on each orientation
    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    // Take the maximum result
    res_1.max(res_2).max(res_3).max(res_4)
}

fn main() {
    // Read n and m from stdin, just like "scanf("%d %d", &n, &m)"
    let stdin = io::stdin();
    let mut first_line = String::new();
    stdin
        .lock()
        .read_line(&mut first_line)
        .expect("Failed to read line for n and m");
    let mut parts = first_line.split_whitespace();
    let n: usize = parts
        .next()
        .expect("Missing n")
        .parse()
        .expect("Failed to parse n as usize");
    let m: usize = parts
        .next()
        .expect("Missing m")
        .parse()
        .expect("Failed to parse m as usize");

    // Read the grid (n rows, each with m integers)
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row_line = String::new();
        stdin
            .lock()
            .read_line(&mut row_line)
            .expect("Failed to read grid row");
        let mut row_values = Vec::with_capacity(m);
        for val_str in row_line.split_whitespace() {
            let val: i32 = val_str.parse().expect("Failed to parse grid value");
            row_values.push(val);
        }
        // Ensure we got exactly m values in this row
        if row_values.len() != m {
            panic!("Expected {} values in a row, got {}", m, row_values.len());
        }
        grid.push(row_values);
    }

    // The original C code uses an array "int gridColSize[1] = {m};"
    // We'll replicate that logic in a small Rust slice
    let grid_col_size = [m];

    // Compute result
    let result = len_of_v_diagonal(&grid, n, &grid_col_size);

    // Print result (same format as printf("%d\n", result))
    println!("{}", result);
}