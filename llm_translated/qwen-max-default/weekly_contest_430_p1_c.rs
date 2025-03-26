use std::io;

fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
    let mut cal_grid = grid.clone();

    let mut ans = 0;
    if grid.len() == 1 {
        return 0;
    }

    for i in 0..grid[0].len() {
        for j in 1..grid.len() {
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() {
    // Read the number of rows and columns
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().expect("Invalid input");
    let grid_col_size: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Initialize the grid and column sizes
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let mut row = Vec::with_capacity(grid_col_size);
        for _ in 0..grid_col_size {
            let mut element = String::new();
            io::stdin().read_line(&mut element).expect("Failed to read line");
            let num: i32 = element.trim().parse().expect("Invalid input");
            row.push(num);
        }
        grid.push(row);
    }

    // Calculate the result
    let result = minimum_operations(&grid);

    // Output the result
    println!("{}", result);
}