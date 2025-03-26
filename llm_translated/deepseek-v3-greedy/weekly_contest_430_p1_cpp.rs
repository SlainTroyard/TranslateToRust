use std::io;

fn main() {
    // Read the dimensions of the grid
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let dimensions: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let m = dimensions[0];
    let n = dimensions[1];

    // Read the grid values
    let mut grid = vec![vec![0; n]; m];
    for i in 0..m {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    // Calculate the minimum operations
    let result = minimum_operations(grid);

    // Print the result
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

    // Iterate through each column
    for i in 0..n {
        // Iterate through each row starting from the second
        for j in 1..m {
            // If the current cell is less than or equal to the cell above it
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // Calculate the required increment
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                // Update the current cell
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}