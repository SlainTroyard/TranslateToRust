use std::io::{self, BufRead};

fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    let grid_col_size = grid[0].len();

    // Clone the grid to avoid modifying the original
    let mut cal_grid = grid.clone();

    if grid_size == 1 {
        return 0;
    }

    let mut ans = 0;

    for i in 0..grid_col_size {
        for j in 1..grid_size {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing gridSize and gridColSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().unwrap();
    let grid_col_size: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![vec![0; grid_col_size]; grid_size];

    // Read the grid values
    for i in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse().unwrap();
        }
    }

    // Compute the result
    let result = minimum_operations(&grid);

    // Output the result
    println!("{}", result);
}