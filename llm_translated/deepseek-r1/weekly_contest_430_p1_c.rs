use std::io::{self, BufRead};

fn minimum_operations(grid: &[Vec<i32>], grid_size: usize, grid_col_sizes: &[usize]) -> i32 {
    let mut cal_grid = grid.to_vec(); // Create a deep copy of the grid
    let mut ans = 0;

    // Early return if there's only one row
    if grid_size == 1 {
        return 0;
    }

    let columns = grid_col_sizes[0]; // Assume all columns are same as first row
    for i in 0..columns {
        for j in 1..grid_size {
            // Ensure each element is strictly greater than the one above
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                let needed = cal_grid[j - 1][i] + 1;
                ans += needed - cal_grid[j][i];
                cal_grid[j][i] = needed;
            }
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read grid dimensions from first line
    let dimensions = lines.next().unwrap();
    let mut parts = dimensions.split_whitespace();
    let grid_size: usize = parts.next().unwrap().parse().unwrap();
    let grid_col_size: usize = parts.next().unwrap().parse().unwrap();

    // Read grid data row by row
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), grid_col_size, "Invalid row length");
        grid.push(row);
    }

    // Create column sizes array (all columns same size)
    let grid_col_sizes = vec![grid_col_size; grid_size];

    // Calculate and print result
    let result = minimum_operations(&grid, grid_size, &grid_col_sizes);
    println!("{}", result);
}