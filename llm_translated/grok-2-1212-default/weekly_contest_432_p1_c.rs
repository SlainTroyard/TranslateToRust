use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let grid_size = grid.len();
    let grid_col_size = grid[0].len();
    let mut ans = Vec::with_capacity((grid_col_size * grid_size + 1) / 2);
    let mut return_size = 0;
    let c1 = grid_col_size - 1 - (grid_col_size & 1);

    for r in 0..grid_size {
        if r & 1 == 1 {
            // Odd rows: traverse from right to left
            let mut c = c1;
            while c >= 0 {
                ans.push(grid[r][c]);
                return_size += 1;
                c -= 2;
            }
        } else {
            // Even rows: traverse from left to right
            let mut c = 0;
            while c < grid_col_size {
                ans.push(grid[r][c]);
                return_size += 1;
                c += 2;
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size and column size
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().unwrap();
    let grid_col_size: usize = iter.next().unwrap().parse().unwrap();

    // Read grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let row: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Perform zigzag traversal
    let ans = zigzag_traversal(&grid);

    // Print result
    for &num in &ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}