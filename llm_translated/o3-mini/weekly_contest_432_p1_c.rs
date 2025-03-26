use std::io::{self, Read};

/// Performs a zigzag traversal on the grid as described in the problem.
///
/// For even-indexed rows (0-indexed), it traverses from left to right,
/// picking every other element starting from index 0.
/// For odd-indexed rows, it traverses from right to left,
/// picking every other element starting from a computed index (c1).
fn zigzag_traversal(grid: &[Vec<i32>], grid_cols: usize) -> Vec<i32> {
    // Calculate c1: grid_cols - 1 - (grid_cols & 1)
    // In C, grid_cols & 1 gives 1 if grid_cols is odd, 0 if even.
    // So, for odd grid_cols, c1 = grid_cols - 2, for even grid_cols, c1 = grid_cols - 1.
    let c1 = grid_cols - 1 - (grid_cols & 1);

    let mut ans = Vec::new();

    // Loop through each row
    for (r, row) in grid.iter().enumerate() {
        if r % 2 == 1 {
            // For odd rows, iterate from c1 down to 0 stepping by 2
            for c in (0..=c1).rev().step_by(2) {
                ans.push(row[c]);
            }
        } else {
            // For even rows, iterate from 0 up to grid_cols stepping by 2
            for c in (0..grid_cols).step_by(2) {
                ans.push(row[c]);
            }
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    // Parse gridSize and gridColSize from the input.
    let grid_size: usize = tokens
        .next()
        .ok_or("Missing grid size")?
        .parse()
        .map_err(|_| "Invalid grid size")?;
    let grid_cols: usize = tokens
        .next()
        .ok_or("Missing grid column size")?
        .parse()
        .map_err(|_| "Invalid grid column size")?;

    // Read the grid values into a Vec<Vec<i32>>.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let mut row = Vec::with_capacity(grid_cols);
        for _ in 0..grid_cols {
            let num: i32 = tokens
                .next()
                .ok_or("Missing grid element")?
                .parse()
                .map_err(|_| "Invalid grid element")?;
            row.push(num);
        }
        grid.push(row);
    }

    // Perform the zigzag traversal.
    let ans = zigzag_traversal(&grid, grid_cols);

    // Print the result in the same format as the C code: each number followed by a space.
    for num in ans {
        print!("{} ", num);
    }
    
    // Ensure the output is flushed.
    io::stdout().flush()?;
    Ok(())
}