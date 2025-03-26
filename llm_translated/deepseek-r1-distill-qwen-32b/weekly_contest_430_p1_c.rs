use std::io;

fn minimum_operations(mut grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    
    let mut ans = 0;
    
    // Iterate through each column
    for c in 0..cols {
        // Iterate from the second row to the end
        for r in 1..rows {
            if grid[r][c] <= grid[r-1][c] {
                let diff = grid[r-1][c] + 1 - grid[r][c];
                ans += diff;
                grid[r][c] = grid[r-1][c] + 1;
            }
        }
    }
    
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut dims = input.split_whitespace();
    let rows: usize = dims.next().expect("No rows").parse().expect("Invalid row count");
    let cols: usize = dims.next().expect("No columns").parse().expect("Invalid column count");
    
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Failed to read row");
        let row_values: Vec<i32> = row.split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        grid.push(row_values);
    }
    
    let result = minimum_operations(grid);
    println!("{}", result);
}