use std::io::{self, BufRead};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let grid_size = grid.len();

    // Process diagonals from main diagonal downward (sorting in descending order)
    for i in 0..grid_size {
        let len = grid_size - i;
        let mut diagonal = Vec::with_capacity(len);
        
        // Extract diagonal elements
        let mut index = 0;
        let mut k = i;
        let mut m = 0;
        while k < grid_size && m < grid_size {
            diagonal.push(grid[k][m]);
            index += 1;
            k += 1;
            m += 1;
        }
        
        // Sort in descending order
        diagonal.sort_by(|a, b| b.cmp(a));
        
        // Put elements back
        index = 0;
        k = i;
        m = 0;
        while k < grid_size && m < grid_size {
            grid[k][m] = diagonal[index];
            index += 1;
            k += 1;
            m += 1;
        }
    }
    
    // Process diagonals from above main diagonal to the right (sorting in ascending order)
    for i in 1..grid_size {
        let len = grid_size - i;
        let mut diagonal = Vec::with_capacity(len);
        
        // Extract diagonal elements
        let mut index = 0;
        let mut k = 0;
        let mut m = i;
        while k < grid_size && m < grid_size {
            diagonal.push(grid[k][m]);
            index += 1;
            k += 1;
            m += 1;
        }
        
        // Sort in ascending order
        diagonal.sort();
        
        // Put elements back
        index = 0;
        k = 0;
        m = i;
        while k < grid_size && m < grid_size {
            grid[k][m] = diagonal[index];
            index += 1;
            k += 1;
            m += 1;
        }
    }
    
    grid
}

fn main() {
    // Read the size of the grid
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the grid values
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Sort the matrix
    let result = sort_matrix(grid);
    
    // Print the result
    for row in &result {
        for (j, val) in row.iter().enumerate() {
            print!("{}", val);
            if j < n - 1 {
                print!(" ");
            }
        }
        println!();
    }
}