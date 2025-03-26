use std::io::{self, BufRead};

fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    let grid_col_size = grid[0].len();
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

    // Read grid size (rows and columns)
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let grid_col_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read grid elements
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Calculate minimum operations
    let result = minimum_operations(grid);

    // Output the result
    println!("{}", result);
}