use std::io;

fn main() {
    // Read the dimensions of the grid
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dimensions: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let m = dimensions[0]; // Number of rows
    let n = dimensions[1]; // Number of columns

    // Read the grid values
    let mut grid = vec![vec![0; n]; m];
    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    // Calculate the minimum number of operations
    let result = minimum_operations(grid);
    println!("{}", result);
}

fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let mut cal_grid = grid.clone();
    let mut ans = 0;
    let m = cal_grid.len();
    let n = cal_grid[0].len();

    // If there's only one row, no operations are needed
    if m == 1 {
        return 0;
    }

    // Iterate through each column and adjust the rows as needed
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