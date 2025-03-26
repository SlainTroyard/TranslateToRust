use std::io;
use std::cmp;

/// Calculates the minimum number of operations required to make columns strictly increasing.
fn minimum_operations(grid: Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> i32 {
    // Create a copy of the grid that we will modify
    let mut cal_grid = grid.clone();
    let mut ans = 0;

    // Edge case: if there's only one row, no operations are needed
    if grid_size == 1 {
        return 0;
    }

    // Iterate over all columns
    for i in 0..grid_col_size {
        for j in 1..grid_size {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // Calculate the number of operations to make the column strictly increasing
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                cal_grid[j][i] = cal_grid[j - 1][i] + 1; // Modify the grid to reflect the operation
            }
        }
    }

    ans
}

fn main() {
    // Read input (grid dimensions and grid values)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let dimensions: Vec<usize> = input.trim().split_whitespace()
                                .map(|x| x.parse::<usize>().expect("Invalid input"))
                                .collect();

    // Extract grid size (number of rows) and gridColSize (number of columns)
    let grid_size = dimensions[0];
    let grid_col_size = dimensions[1];

    // Initialize the grid
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _ in 0..grid_size {
        let mut row_input = String::new();
        io::stdin().read_line(&mut row_input).expect("Failed to read row input");
        let row: Vec<i32> = row_input.trim().split_whitespace()
                            .map(|x| x.parse::<i32>().expect("Invalid input"))
                            .collect();
        grid.push(row);
    }

    // Call the function and print the result
    let result = minimum_operations(grid, grid_size, grid_col_size);
    println!("{}", result);
}