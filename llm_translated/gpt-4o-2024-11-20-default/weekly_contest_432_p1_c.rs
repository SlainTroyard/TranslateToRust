use std::io::{self, Write};

// Translates the zigzagTraversal function from C to Rust
fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let grid_size = grid.len();
    if grid_size == 0 {
        return vec![];
    }
    let grid_col_size = grid[0].len();
    let mut result = Vec::with_capacity((grid_col_size * grid_size + 1) / 2);
    for r in 0..grid_size {
        if r % 2 == 1 {
            // Odd row: traverse right-to-left, skipping by 2
            for c in (0..grid_col_size).rev().filter(|&x| x % 2 == 1) {
                result.push(grid[r][c]);
            }
        } else {
            // Even row: traverse left-to-right, skipping by 2
            for c in (0..grid_col_size).filter(|&x| x % 2 == 0) {
                result.push(grid[r][c]);
            }
        }
    }
    result
}

fn main() {
    // Reading input according to the format specified in the C program
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dimensions: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expecting an integer"))
        .collect();

    let grid_size = dimensions[0];
    let grid_col_size = dimensions[1];

    let mut grid = Vec::with_capacity(grid_size);

    for _ in 0..grid_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Expecting an integer"))
            .collect();
        grid.push(row);
    }

    // Call the zigzag_traversal function on the given grid
    let result = zigzag_traversal(grid);

    // Output the result in the same format as the original C program (space-separated integers)
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, value) in result.iter().enumerate() {
        write!(handle, "{}", value).unwrap();
        if i != result.len() - 1 {
            write!(handle, " ").unwrap();
        }
    }
    writeln!(handle).unwrap();
}