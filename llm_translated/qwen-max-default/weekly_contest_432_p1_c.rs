use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    let c1 = grid_col_size - 1 - (grid_col_size % 2);
    
    for (r, row) in grid.iter().enumerate() {
        if r & 1 == 1 {
            for c in (0..=c1).rev().step_by(2) {
                ans.push(row[c]);
            }
        } else {
            for c in (0..grid_col_size).step_by(2) {
                ans.push(row[c]);
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get gridSize and gridColSize
    let line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Expected input"))??;
    let mut iter = line.split_whitespace();
    let grid_size: usize = iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Expected gridSize"))?.parse()?;
    let grid_col_size: usize = iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Expected gridColSize"))?.parse()?;

    // Initialize the grid
    let mut grid = vec![vec![0; grid_col_size]; grid_size];

    // Fill the grid with values from stdin
    for i in 0..grid_size {
        let line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Expected grid row"))??;
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse::<i32>()?;
        }
    }

    // Perform the zigzag traversal
    let ans = zigzag_traversal(&grid, grid_col_size);

    // Print the result
    for value in ans {
        print!("{} ", value);
    }
    println!();

    Ok(())
}