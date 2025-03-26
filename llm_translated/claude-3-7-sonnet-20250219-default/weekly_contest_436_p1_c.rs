use std::io::{self, BufRead};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let grid_size = grid.len();
    
    // Process diagonals starting from the main diagonal and moving right
    for i in 0..grid_size {
        let len = grid_size - i;
        let mut diag_values = Vec::with_capacity(len);
        
        // Extract values from the diagonal
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            diag_values.push(grid[k][m]);
        }
        
        // Sort in descending order
        diag_values.sort_by(|a, b| b.cmp(a));
        
        // Put values back
        let mut index = 0;
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            grid[k][m] = diag_values[index];
            index += 1;
        }
    }
    
    // Process diagonals starting from one above the main diagonal and moving up
    for i in 1..grid_size {
        let len = grid_size - i;
        let mut diag_values = Vec::with_capacity(len);
        
        // Extract values from the diagonal
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            diag_values.push(grid[k][m]);
        }
        
        // Sort in ascending order
        diag_values.sort();
        
        // Put values back
        let mut index = 0;
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            grid[k][m] = diag_values[index];
            index += 1;
        }
    }
    
    grid
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid size
    let n: usize = lines.next().unwrap()?.trim().parse().expect("Expected a number for grid size");
    
    // Read the grid
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Expected a number"))
            .collect();
        grid.push(row);
    }
    
    // Sort the matrix
    let result = sort_matrix(grid);
    
    // Output the result
    for row in &result {
        for (j, val) in row.iter().enumerate() {
            if j > 0 {
                print!(" ");
            }
            print!("{}", val);
        }
        println!();
    }
    
    Ok(())
}