use std::io;

fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let grid_size = grid.len();

    if grid_size <= 1 {
        return 0;
    }

    if grid_size > 0 && !grid[0].is_empty() {
        let grid_col_size = grid[0].len();
        for i in 0..grid_col_size {
            for j in 1..grid_size {
                if grid[j][i] <= grid[j - 1][i] {
                    ans += grid[j - 1][i] + 1 - grid[j][i];
                    grid[j][i] = grid[j - 1][i] + 1;
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut iter = input_line.trim().split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().expect("Invalid input for gridSize");
    let grid_col_size: usize = iter.next().unwrap().parse().expect("Invalid input for gridColSize");

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let mut row: Vec<i32> = Vec::with_capacity(grid_col_size);
        let mut input_row = String::new();
        io::stdin().read_line(&mut input_row).expect("Failed to read line");
        for s in input_row.trim().split_whitespace() {
            row.push(s.parse().expect("Invalid input for grid element"));
        }
        grid.push(row);
    }

    let result = minimum_operations(&mut grid);
    println!("{}", result);
}