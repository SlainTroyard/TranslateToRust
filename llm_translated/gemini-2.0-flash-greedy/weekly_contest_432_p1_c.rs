use std::io;
use std::io::Read;
use std::str::FromStr;

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::with_capacity((grid_col_size * grid_size + 1) / 2);
    let mut r = 0;
    let mut c: usize;
    let mut c1 = grid_col_size - 1 - (grid_col_size & 1);

    while r < grid_size {
        if r & 1 == 1 {
            // Odd rows: traverse from c1 down to 0 with step 2
            c = c1;
            while c >= 0 {
                ans.push(grid[r][c]);
                if c < 2 {
                    break;
                }
                c -= 2;
            }
        } else {
            // Even rows: traverse from 0 up to grid_col_size with step 2
            c = 0;
            while c < grid_col_size {
                ans.push(grid[r][c]);
                c += 2;
            }
        }
        r += 1;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_whitespace();
    let grid_size = first_line.next().unwrap().parse::<usize>()?;
    let grid_col_size = first_line.next().unwrap().parse::<usize>()?;

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let row_line = lines.next().unwrap();
        let row: Vec<i32> = row_line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        grid.push(row);
    }

    let ans = zigzag_traversal(&grid, grid_size, grid_col_size);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}