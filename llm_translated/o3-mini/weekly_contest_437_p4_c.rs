use std::cmp;
use std::error::Error;
use std::io::{self, Read};

/// Check for success condition between two values:
/// Returns true if a and b are (0, 2) or (2, 0)
fn suc(a: i32, b: i32) -> bool {
    (a == 0 && b == 2) || (a == 2 && b == 0)
}

/// Update the dp table along the upper-right diagonal starting from (i, j).
/// dp is a mutable reference to a table with dimensions m×n.
///
/// This function mimics the C `update_dp` function:
/// It sets dp[i][j] to 1 and moves diagonally upward-right (`i--`, `j++`) while
/// checking if the current diagonal element follows the sequence condition.
fn update_dp(
    grid: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
    m: usize,
    n: usize,
    mut i: isize,
    mut j: isize,
) {
    dp[i as usize][j as usize] = 1;
    i -= 1;
    j += 1;
    while i >= 0 && (j as usize) < n {
        // Check the success condition using the element below-left as previous element.
        if suc(grid[i as usize][j as usize], grid[(i + 1) as usize][(j - 1) as usize]) {
            dp[i as usize][j as usize] = dp[(i + 1) as usize][(j - 1) as usize] + 1;
        } else {
            dp[i as usize][j as usize] = 1;
        }
        i -= 1;
        j += 1;
    }
}

/// Solve function for one orientation of the grid.
/// This function replicates the C `solve` method exactly.
///
/// The grid is of dimensions m×n, and we use dynamic programming (dp) to store
/// the maximum consecutive diagonal "chain" length at each cell.
fn solve(grid: &Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    // Create dp table with the same dimensions as the grid.
    let mut dp = vec![vec![0; n]; m];

    // Update dp table for diagonals starting from the left column
    for i in 0..m {
        update_dp(grid, &mut dp, m, n, i as isize, 0);
    }
    // Update dp table for diagonals starting from the bottom row (except the first column)
    for j in 1..n {
        update_dp(grid, &mut dp, m, n, (m - 1) as isize, j as isize);
    }

    let mut ans = 0;
    // Loop over each cell in the grid
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = cmp::max(ans, 1);
                let mut ii = i + 1;
                let mut jj = j + 1;
                let mut len = 1; // Initially, the length is 1.
                // Explore in the lower-right direction.
                while ii < m && jj < n {
                    if len == 1 && grid[ii][jj] != 2 {
                        break;
                    }
                    if len > 1 && !suc(grid[ii][jj], grid[ii - 1][jj - 1]) {
                        break;
                    }
                    ans = cmp::max(ans, len as i32 + dp[ii][jj]);
                    len += 1;
                    ii += 1;
                    jj += 1;
                }
            }
        }
    }
    ans
}

/// Rotate the grid 90 degrees clockwise.
///
/// The rotated grid has dimensions n×m if the original grid has dimensions m×n.
/// The new grid's element at (i, j) is grid[j][n-1-i].
fn rotate(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut new_grid = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            new_grid[i][j] = grid[j][n - 1 - i];
        }
    }
    new_grid
}

/// Compute the maximum length of valid diagonal sequences across all four orientations.
/// This function replicates the C `lenOfVDiagonal` function exactly.
fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    // Rotate the grid three times to cover all four orientations.
    let arr_1 = rotate(grid);
    let arr_2 = rotate(&arr_1);
    let arr_3 = rotate(&arr_2);

    // Compute the result (maximum valid diagonal sequence length) for each orientation.
    let res_1 = solve(grid, m, n);
    let res_2 = solve(&arr_1, n, m);
    let res_3 = solve(&arr_2, m, n);
    let res_4 = solve(&arr_3, n, m);

    cmp::max(cmp::max(res_1, res_2), cmp::max(res_3, res_4))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Use whitespace-based tokenization.
    let mut tokens = input.split_whitespace();

    // The first two values are n and m (grid dimensions).
    let n: usize = tokens
        .next()
        .ok_or("Missing number of rows")?
        .parse()
        .map_err(|_| "Invalid input for number of rows")?;
    let m: usize = tokens
        .next()
        .ok_or("Missing number of columns")?
        .parse()
        .map_err(|_| "Invalid input for number of columns")?;

    // Read the grid. The grid contains n rows with m integers each.
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(m);
        for _ in 0..m {
            let val: i32 = tokens
                .next()
                .ok_or("Missing grid element")?
                .parse()
                .map_err(|_| "Invalid grid element")?;
            row.push(val);
        }
        grid.push(row);
    }

    // Compute the result using the len_of_v_diagonal function.
    let res = len_of_v_diagonal(&grid);
    // Output the result.
    println!("{}", res);
    Ok(())
}