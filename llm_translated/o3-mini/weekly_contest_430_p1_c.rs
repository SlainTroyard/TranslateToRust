use std::io::{self, BufRead};

/// This function calculates the minimum number of operations required
/// so that each column in the grid is strictly increasing (from top to bottom).
///
/// The logic is as follows:
/// 1. Create a copy of the grid (cal_grid).
/// 2. For each column, traverse from the second row onward and if the value in the current
///    row is not greater than the value in the previous row, increment it just enough to make it larger.
/// 3. Count the total number of increments made and return the result.
fn minimum_operations(mut grid: Vec<Vec<i32>>) -> i32 {
    // Clone grid into cal_grid to work on it.
    let mut cal_grid = grid.clone();

    let grid_size = cal_grid.len();
    if grid_size == 0 {
        return 0;
    }
    let cols = cal_grid[0].len();
    let mut ans = 0;

    // If there is only one row, no operations are needed.
    if grid_size == 1 {
        return 0;
    }

    // Iterate over each column.
    for i in 0..cols {
        // For each cell except the first row, ensure the column is strictly increasing.
        for j in 1..grid_size {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // Calculate the needed increment.
                let increment = cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                ans += increment;
                // Update the current cell in cal_grid.
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // The first two integers specify the grid dimensions:
    // grid_size (number of rows) and grid_col_size (number of columns per row).
    let mut numbers = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        // Skip empty lines.
        if line.trim().is_empty() {
            continue;
        }
        // Parse all numbers in this line.
        for token in line.split_whitespace() {
            match token.parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(e) => {
                    eprintln!("Error parsing number: {}", e);
                    std::process::exit(1);
                }
            }
        }
        // Break once we have at least 2 numbers (grid dimensions).
        if numbers.len() >= 2 {
            break;
        }
    }

    // Ensure that we have at least two numbers for grid dimensions.
    if numbers.len() < 2 {
        eprintln!("Not enough input for grid dimensions.");
        std::process::exit(1);
    }

    // Extract grid dimensions.
    let grid_size = numbers[0] as usize;
    let grid_col_size = numbers[1] as usize;

    // Read the remaining numbers which represent the grid elements.
    // There should be grid_size * grid_col_size integers.
    let mut remaining_nums = numbers[2..].to_vec(); // Any numbers left on the first line.
    while remaining_nums.len() < grid_size * grid_col_size {
        if let Some(Ok(line)) = lines.next() {
            if line.trim().is_empty() {
                continue;
            }
            for token in line.split_whitespace() {
                match token.parse::<i32>() {
                    Ok(num) => remaining_nums.push(num),
                    Err(e) => {
                        eprintln!("Error parsing number: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        } else {
            break;
        }
    }

    // Ensure that we have received enough numbers for the grid.
    if remaining_nums.len() < grid_size * grid_col_size {
        eprintln!("Not enough numbers for grid.");
        std::process::exit(1);
    }

    // Build the 2D grid from the list of numbers.
    let mut grid = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        let start = i * grid_col_size;
        let end = start + grid_col_size;
        let row = remaining_nums[start..end].to_vec();
        grid.push(row);
    }

    // Calculate the answer and output it.
    let result = minimum_operations(grid);
    println!("{}", result);
}