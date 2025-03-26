use std::io::{self, Read};

fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    let grid_col_size = if grid_size > 0 { grid[0].len() } else { 0 };

    let mut ans = 0;
    if grid_size == 1 {
        return 0;
    }

    for col in 0..grid_col_size {
        for row in 1..grid_size {
            if grid[row][col] <= grid[row - 1][col] {
                ans += grid[row - 1][col] + 1 - grid[row][col];
                grid[row][col] = grid[row - 1][col] + 1;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let grid_size = first_line[0] as usize;
    let grid_col_size = first_line[1] as usize;

    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let row: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    let result = minimum_operations(&mut grid);
    println!("{}", result);

    Ok(())
}