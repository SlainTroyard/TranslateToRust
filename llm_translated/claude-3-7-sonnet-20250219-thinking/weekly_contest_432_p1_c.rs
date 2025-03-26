use std::io::{self, BufRead};

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    // Allocate space for result array
    let capacity = ((grid_col_size * grid_size + 1) >> 1);
    let mut ans = vec![0; capacity];
    let mut return_size = 0;
    let c1 = grid_col_size as i32 - 1 - ((grid_col_size & 1) as i32);
    
    let mut r = 0;
    while r < grid_size {
        if r & 1 != 0 {
            // For odd rows, start from c1 and move backwards
            let mut c = c1;
            while c >= 0 {
                ans[return_size] = grid[r][c as usize];
                return_size += 1;
                c -= 2;
            }
        } else {
            // For even rows, start from 0 and move forward
            let mut c = 0;
            while c < grid_col_size {
                ans[return_size] = grid[r][c];
                return_size += 1;
                c += 2;
            }
        }
        r += 1;
    }
    
    // Return only the filled part of the array
    ans[0..return_size].to_vec()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Read gridSize and gridColSize
    let first_line = iterator.next().unwrap()?;
    let mut values = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let grid_size = values.next().unwrap();
    let grid_col_size = values.next().unwrap();
    
    // Allocate and read the grid
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = iterator.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Process the grid
    let ans = zigzag_traversal(&grid, grid_size, grid_col_size);
    
    // Print the result
    for (i, &val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}