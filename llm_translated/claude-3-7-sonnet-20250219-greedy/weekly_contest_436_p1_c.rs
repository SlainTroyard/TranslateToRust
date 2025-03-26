use std::io::{self, BufRead};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let grid_size = grid.len();
    
    // Sort diagonals starting from the main diagonal and below (descending order)
    for i in 0..grid_size {
        let mut diagonal_values = Vec::new();
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            if m < grid_size {
                diagonal_values.push(grid[k][m]);
            }
        }
        
        // Sort in descending order
        diagonal_values.sort_by(|a, b| b.cmp(a));
        
        let mut index = 0;
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            if m < grid_size {
                grid[k][m] = diagonal_values[index];
                index += 1;
            }
        }
    }
    
    // Sort diagonals above the main diagonal (ascending order)
    for i in 1..grid_size {
        let mut diagonal_values = Vec::new();
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            if k < grid_size && m < grid_size {
                diagonal_values.push(grid[k][m]);
            }
        }
        
        // Sort in ascending order
        diagonal_values.sort();
        
        let mut index = 0;
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            if k < grid_size && m < grid_size {
                grid[k][m] = diagonal_values[index];
                index += 1;
            }
        }
    }
    
    grid
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the matrix
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the matrix
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Sort the matrix
    let result = sort_matrix(grid);
    
    // Print the result
    for row in result {
        for (j, val) in row.iter().enumerate() {
            print!("{}", val);
            if j < n - 1 {
                print!(" ");
            }
        }
        println!();
    }
}