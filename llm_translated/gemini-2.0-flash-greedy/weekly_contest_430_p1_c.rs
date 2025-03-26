use std::io;
use std::io::Read;

fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
    let grid_size = grid.len();
    let grid_col_size = if grid_size > 0 { grid[0].len() } else { 0 };

    let mut ans = 0;
    if grid_size == 1 {
        return 0;
    }

    for i in 0..grid_col_size {
        for j in 1..grid_size {
            if grid[j][i] <= grid[j - 1][i] {
                ans += grid[j - 1][i] + 1 - grid[j][i];
                grid[j][i] = grid[j - 1][i] + 1;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let grid_size: usize = first_line_split.next().unwrap().parse().unwrap();
    let grid_col_size: usize = first_line_split.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let result = minimum_operations(&mut grid);
    println!("{}", result);

    Ok(())
}