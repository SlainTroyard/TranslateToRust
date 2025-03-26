// Problem: Weekly Contest 430 Problem 1

use std::error::Error;
use std::io::{self, BufRead};

/// Returns the minimum number of operations required to make
/// each row element strictly larger than the element above it.
fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
    let mut cal_grid = grid.clone(); // make a mutable copy of the original grid
    let mut ans = 0;
    let m = cal_grid.len();
    let n = cal_grid[0].len();

    // If there is only one row, no operations are needed
    if m == 1 {
        return 0;
    }

    // For each column, compare current row element with the element above.
    // If it's not strictly larger, increase it just enough to make it larger.
    for i in 0..n {
        for j in 1..m {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read m and n (number of rows and columns) from stdin
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    let tokens = first_line.split_whitespace().collect::<Vec<_>>();
    let m = tokens[0].parse::<usize>()?;
    let n = tokens[1].parse::<usize>()?;

    // Read the grid from stdin
    let mut grid = Vec::with_capacity(m);
    for _ in 0..m {
        let mut row_input = String::new();
        io::stdin().read_line(&mut row_input)?;
        let values = row_input.split_whitespace().map(|x| x.parse::<i32>());

        let mut row = Vec::with_capacity(n);
        for val in values {
            row.push(val?);
        }

        // Check if the row has the required number of columns
        if row.len() != n {
            return Err("Invalid input format: not enough values in a row".into());
        }
        grid.push(row);
    }

    // Compute and print the result (minimum operations)
    let ans = minimum_operations(&grid);
    println!("{}", ans);

    Ok(())
}