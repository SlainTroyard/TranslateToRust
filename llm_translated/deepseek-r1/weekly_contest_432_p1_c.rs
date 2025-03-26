use std::io::{self, BufRead};

fn zigzag_traversal(grid: &[Vec<i32>], grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::with_capacity((grid_size * grid_col_size + 1) / 2);
    
    for r in 0..grid_size {
        if r % 2 == 0 {
            // Even row: collect elements starting from column 0, step by 2
            for c in (0..grid_col_size).step_by(2) {
                ans.push(grid[r][c]);
            }
        } else {
            // Odd row: calculate starting column and collect elements stepping back by 2
            let c1 = (grid_col_size - 1) - (grid_col_size % 2);
            for c in (0..=c1).rev().step_by(2) {
                ans.push(grid[r][c]);
            }
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    
    // Read grid dimensions
    let first_line = lines.next().expect("Missing first line");
    let mut dims = first_line.split_whitespace();
    let grid_size: usize = dims.next().expect("Missing grid size")
        .parse().expect("Grid size must be an integer");
    let grid_col_size: usize = dims.next().expect("Missing grid column size")
        .parse().expect("Grid column size must be an integer");
    
    // Read grid data
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().expect("Insufficient grid rows");
        let row: Vec<i32> = line.split_whitespace()
            .take(grid_col_size)
            .map(|s| s.parse().expect("Grid element must be an integer"))
            .collect();
        assert_eq!(row.len(), grid_col_size, "Invalid row length");
        grid.push(row);
    }
    
    // Perform zigzag traversal
    let result = zigzag_traversal(&grid, grid_size, grid_col_size);
    
    // Print result with space-separated format
    for num in result {
        print!("{} ", num);
    }
    println!();
}